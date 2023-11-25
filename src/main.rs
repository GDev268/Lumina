use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use lumina_core::{device::Device, fps_manager::FPS, window::Window};

use lumina_debug::logger::{Logger, SeverityLevel};
use lumina_geometry::shapes::{self};
use lumina_graphic::renderer::Renderer;
use lumina_input::{
    keyboard::{Keyboard, Keycode},
    mouse::{Mouse, MouseButton},
};
use lumina_object::{game_object::GameObject, transform::Transform};
//use lumina_pbr::material::Material;
use lumina_render::camera::Camera;
use lumina_scene::query::Query;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let win = WindowBuilder::new()
        .build(&event_loop)
        .expect("window build fail");

    let mut window = Window::new(&event_loop, "Lumina Dev App", 800, 640);
    let mut device = Device::new(&window, wgpu::Backends::all()).await;

    let mut renderer = Renderer::new(&device, &window);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { window_id, event } if window_id == win.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::Resized(physical_size) => {
                    device.resize(physical_size);
                    window.resize(physical_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    device.resize(*new_inner_size);
                    window.resize(*new_inner_size);
                }

                WindowEvent::CursorMoved { .. } | WindowEvent::KeyboardInput { .. } => {}

                _ => {}
            },

            Event::RedrawRequested(window_id) if window_id == win.id() => {
                renderer.begin_frame(&mut device);
            }

            Event::MainEventsCleared => {
                win.request_redraw();
            }

            _ => {}
        }
        renderer.update(&device, &window);
    });
}
