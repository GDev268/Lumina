/*use std::{cell::RefCell, rc::Rc};

use sdl2::render;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use lumina_core::{device::Device, fps_manager::FPS, window::Windo};

use lumina_debug::logger::{Logger, SeverityLevel};
use lumina_geometry::shapes::{self};
use lumina_graphic::{
    pipeline::{Pipeline, PipelineConfiguration},
    renderer::Renderer,
    shader::Shader,
};
use lumina_input::{
    keyboard::{Keyboard, Keycode},
    mouse::{Mouse, MouseButton},
};
use lumina_object::{game_object::GameObject, transform::Transform};
//use lumina_pbr::material::Material;
use lumina_render::{camera::Camera, mesh::Mesh};
use lumina_scene::query::Query;*/

/*use std::{cell::RefCell, iter, rc::Rc};

use cgmath::prelude::*;
use lumina_core::{device::Device, window::Window, Vertex};
use lumina_graphic::{
    pipeline::{Pipeline, PipelineConfiguration},
    renderer::Renderer,
    shader::Shader,
};
use lumina_render::mesh::Mesh;
use wgpu::include_spirv_raw;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.0, 0.0, 1.0],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.0, 0.0, 0.1],
    }, // E
];

const INDICES: &[u32] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut window = Window::new(&event_loop, "Lumina Test", 860, 640);
    let device: Rc<RefCell<Device>> = Rc::new(RefCell::new(
        Device::new(&window, wgpu::Backends::PRIMARY).await,
    ));

    let shader = Shader::new(&device.borrow(), "shaders/default.wgsl");

    let render_pipeline_layout =
        device
            .borrow()
            .device()
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

    let mut pipeline_config = PipelOineConfiguration::default();
    pipeline_config.pipeline_layout = Some(render_pipeline_layout);

    let render_pipeline = Pipeline::new(&device.borrow(), &shader, &pipeline_config, "0");
    let mesh = Mesh::new(&device.borrow(), VERTICES.to_vec(), INDICES.to_vec());
    let renderer = Renderer::new(Rc::clone(&device), &window);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window._window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        window.resize(*physical_size);
                        device.borrow_mut().resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        window.resize(**new_inner_size);
                        device.borrow_mut().resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window._window.id() => {
                renderer.render_object(&shader,&mesh,&render_pipeline);
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window._window.request_redraw();
            }
            _ => {}
        }
    });
}*/

use glsl_parser::parser::Parser;
use lumina_graphic::shader::Shader;
use lumina_core::{device::Device,window::Window};
use std::{cell::RefCell, rc::Rc};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};


fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut window = Window::new(&event_loop, "Lumina Test", 860, 640);
    let device: Rc<RefCell<Device>> = Rc::new(RefCell::new(
        Device::new(&window, wgpu::Backends::PRIMARY).await,
    ));
    Shader::new(&device.borrow(),"shaders/default.wgsl");
    let mut parser = Parser::new();
    parser.parse_shader("shaders/default.wgsl");
}
