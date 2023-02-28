use bytemuck::bytes_of;
use std::borrow::Cow;
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
pub struct Uniform {
    tiles_x: u32,
    gap: f32,
    step_size: f32,
    mouse: [f32; 2],
    mouse_speed: f32,
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
    instances: u32,
    instances_strength: Vec<InstanceStrength>,
    instances_color_range: Vec<InstanceColorRange>,
    instance_buffer_strength: wgpu::Buffer,
    instance_buffer_color_range: wgpu::Buffer,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
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

        let vertex_array = [
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

        let num_vertices = vertex_array.len() as u32;

        // use wgpu::util::DeviceExt;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_array),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let instances = 32;
        let instances_strength = vec![InstanceStrength { value: 1.0 }; instances];
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
            instances
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
            vertex_array,
            instances: instances as u32,
            instances_strength,
            instances_color_range,
            instance_buffer_strength,
            instance_buffer_color_range,
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

    pub fn update(&mut self) {}

    pub fn change(&mut self, with: f32) {
        {
            let x = &mut self.vertex_array[0].position[0];
            if 1.0 < *x {
                *x -= 2.0;
            }
            *x += with;
        }
        {
            let y = &mut self.vertex_array[2].position[1];
            if 1.0 < *y {
                *y -= 2.0;
            }
            *y += with * 0.5;
        }
        let size = self.instances_strength.len();
        for (i, v) in self.instances_strength.iter_mut().enumerate() {
            v.value -= 0.1 * i as f32 / size as f32;
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
            rpass.draw(0..self.num_vertices, 0..self.instances);
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
        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
