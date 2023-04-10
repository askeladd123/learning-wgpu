// todo: change fading to happen in compute shader
// todo: add support for transparancy
use crate::color::Color;
use bytemuck::bytes_of;
use log::{debug, error, info, trace, warn};
use std::borrow::Cow;
use std::convert::{From, TryFrom};
use std::iter::IntoIterator;
use wgpu::util::DeviceExt;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3], // todo: don't need this field
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceStrength {
    value: f32,
}

impl InstanceStrength {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceStrength>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                shader_location: 5,
                format: wgpu::VertexFormat::Float32,
            }],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceColorRange {
    high: [f32; 3],
    low: [f32; 3],
    // high: Color,
    // low: Color,
    // _padding: [f32; 15],
    // speed: f32,
}

impl InstanceColorRange {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceColorRange>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
    pub tiles_x: u32,
    pub gap: f32,
    pub margin: f32,
    pub speed: f32,
    pub mouse_speed: f32,
    // apparently uniforms requires 16 byte (4 float) spacing,
    // so padding has to be this size, and this location pub _padding: f32,
    _padding: f32,
    pub mouse: [f32; 2],
}

impl Default for Uniform {
    fn default() -> Self {
        Self {
            tiles_x: 6,
            gap: 0.05,
            margin: 0.02,
            speed: 1.0,
            mouse_speed: 0.0,
            mouse: [0.; 2],
            _padding: 0.0,
        }
    }
}

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    window: Window,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    pub vertex_array: [Vertex; 6],
    pub size: winit::dpi::PhysicalSize<u32>,
    w: u32,
    h: u32,
    instances_strength: Vec<InstanceStrength>,
    instances_color_range: Vec<InstanceColorRange>,
    instance_buffer_strength: wgpu::Buffer,
    instance_buffer_color_range: wgpu::Buffer,
    uniform: Uniform,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let (w, h) = (128, 128);
        let instances = w * h;
        let uniform = Uniform {
            tiles_x: w,
            ..Uniform::default()
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                    InstanceStrength::desc(),
                    InstanceColorRange::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        const VERTEX_ARRAY: [Vertex; 6] = [
            // tri w
            // wn
            Vertex {
                position: [-0.5, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            // ws
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            // en
            Vertex {
                position: [0.5, 0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
            // tri e
            // ws
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            // es
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            // en
            Vertex {
                position: [0.5, 0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let num_vertices = VERTEX_ARRAY.len() as u32;

        // use wgpu::util::DeviceExt;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&VERTEX_ARRAY),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let instances_strength = vec![InstanceStrength { value: 0.0 }; instances as usize];
        let instance_buffer_strength =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("instance buffer strenght"),
                contents: bytemuck::cast_slice(&instances_strength),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        let instances_color_range = vec![
            InstanceColorRange {
                high: [0.9, 0.9, 0.9],
                low: [0.1, 0.1, 0.1],
            };
            instances as usize
        ];
        let instance_buffer_color_range =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("instance buffer color range"),
                contents: bytemuck::cast_slice(&instances_color_range),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            size,
            window,
            vertex_buffer,
            num_vertices,
            vertex_array: VERTEX_ARRAY,
            w,
            h,
            instances_strength,
            instances_color_range,
            instance_buffer_strength,
            instance_buffer_color_range,
            uniform,
            uniform_buffer,
            bind_group,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        for tile in self.instances_strength.iter_mut() {
            tile.value += 0.01;
        }
    }

    pub fn get_uniform(&mut self) -> &mut Uniform {
        &mut self.uniform
    }

    pub fn change(&mut self, with: f32) {
        // {
        //     let x = &mut self.vertex_array[0].position[0];
        //     if 1.0 < *x {
        //         *x -= 2.0;
        //     }
        //     *x += with;
        // }
        // {
        //     let y = &mut self.vertex_array[2].position[1];
        //     if 1.0 < *y {
        //         *y -= 2.0;
        //     }
        //     *y += with * 0.5;
        // }
        // let size = self.instances_strength.len();
        // for (i, v) in self.instances_strength.iter_mut().enumerate() {
        //     v.value -= 0.1 * i as f32 / size as f32;
        // }
        for tile in self.instances_strength.iter_mut() {
            tile.value -= with;
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.set_vertex_buffer(1, self.instance_buffer_strength.slice(..));
            rpass.set_vertex_buffer(2, self.instance_buffer_color_range.slice(..));
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw(0..self.num_vertices, 0..(self.w * self.h));
        }

        self.queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&self.vertex_array),
        );
        self.queue.write_buffer(
            &self.instance_buffer_strength,
            0,
            bytemuck::cast_slice(&self.instances_strength),
        );
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniform]),
        );
        self.queue.write_buffer(
            &self.instance_buffer_color_range,
            0,
            bytemuck::cast_slice(&self.instances_color_range),
        );
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    /// panics if tile is out of bounds
    pub fn paint(&mut self, tile: Tile) {
        if self.w <= tile.x || self.h <= tile.y {
            // dbg!(self.uniform.tiles_x, self.instances_color_range.len());
            panic!("tile provided was out of bounds, \n\twidth: {}, tile.x: {}\n\theight: {}, tile.y: {}", self.w, tile.x, self.h, tile.y);
        }

        let index = (tile.x + tile.y * self.w) as usize;
        let mut i: &mut InstanceColorRange = &mut self.instances_color_range[index];
        i.high = tile.high.into();
        i.low = tile.low.into();
        self.instances_strength[index].value = 0.0;
    }

    pub fn tiles_w(&self) -> u32 {
        self.w
    }
    pub fn tiles_h(&self) -> u32 {
        self.h
    }
}

pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub high: Color,
    pub low: Color,
    pub speed: f32,
}

impl Tile {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            high: Color::WHITE,
            low: Color::BLACK,
            speed: 1.0,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            high: Color::WHITE,
            low: Color::BLACK,
            speed: 1.0,
        }
    }
}
