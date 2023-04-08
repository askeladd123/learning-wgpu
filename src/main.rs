#![allow(unused)]
use cfg_if::cfg_if;
use log::{debug, error, info, trace, warn};
use std::default::Default;
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod color;
mod graphics;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(run());
    // todo: do I need wasm_bindgen_futures::spawn_local()? or can wasm functions be async?
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
async fn run() {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")]{
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Trace);
        } else {
            env_logger::init();
        }
    }
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }

    let mut gfx = graphics::State::new(window).await;

    gfx.paint(graphics::Tile {
        x: 1,
        y: 1,
        high: color::Color::RED,
        low: color::Color::BLUE,
        ..graphics::Tile::default()
    });
    gfx.paint(graphics::Tile {
        x: 2,
        y: 2,
        high: color::Color::GREEN,
        low: color::Color::GREY,
        ..graphics::Tile::default()
    });

    gfx.paint(graphics::Tile {
        x: 3,
        y: 3,
        ..graphics::Tile::default()
    });

    gfx.paint(graphics::Tile {
        x: 4,
        y: 4,
        ..graphics::Tile::default()
    });

    gfx.paint(graphics::Tile {
        x: 5,
        y: 5,
        ..graphics::Tile::default()
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                gfx.resize(size);
            }
            Event::RedrawRequested(window_id) if window_id == gfx.window().id() => {
                gfx.update();
                match gfx.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => gfx.resize(gfx.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                gfx.get_uniform().mouse = [
                    position.x as f32 / gfx.window().inner_size().width as f32,
                    position.y as f32 / gfx.window().inner_size().height as f32,
                ];
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Up),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                gfx.change(0.05);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Down),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                gfx.change(-0.05);
            }
            Event::MainEventsCleared => {
                gfx.window().request_redraw();
            }
            _ => {}
        }
    })
}
