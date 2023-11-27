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


use std::{iter, rc::Rc, cell::RefCell};

use cgmath::prelude::*;
use lumina_core::{device::Device,window::Window, Vertex};
use lumina_graphic::{shader::Shader, pipeline::{PipelineConfiguration, Pipeline}, renderer::Renderer};
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
        color: [0.5, 0.0, 1.0],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 1.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.75],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.865, 0.0, 0.5],
    }, // E
];

const INDICES: &[u32] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

struct State {
    device:Rc<RefCell<Device>>,
    render_pipeline: Pipeline,

    /*vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,*/
    num_indices: u32,

    mesh:Mesh
}

impl State {
    async fn new(window: &Window) -> Self {
        let device: Rc<RefCell<Device>> = Rc::new(RefCell::new(Device::new(window,wgpu::Backends::PRIMARY).await));

        let shader = Shader::new(&device.borrow(),"shaders/default.wgsl");

        let render_pipeline_layout =
            device.borrow().device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let mut pipeline_config = PipelineConfiguration::default();
        pipeline_config.pipeline_layout = Some(render_pipeline_layout);


        let render_pipeline = Pipeline::new(&device.borrow(),&shader,&pipeline_config,"0");
        let mesh = Mesh::new(&device.borrow(), VERTICES.to_vec(), INDICES.to_vec());

        let num_indices = INDICES.len() as u32;

        Self {
            device,
            render_pipeline,
            /*vertex_buffer,
            index_buffer,*/
            num_indices,
            mesh
        }
    }


    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            _ => false,
        }
    }

    fn update(&mut self) {}

    fn render(&mut self) {
        let output = self.device.borrow().get_surface().get_current_texture().expect("Faield to get the surface");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device.borrow().device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline.graphics_pipeline);

            render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.mesh.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.device.borrow().queue.submit(iter::once(encoder.finish()));
        output.present();

    }
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let mut window = Window::new(&event_loop, "Lumina Test", 860, 640);

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window._window.id() => {
                if !state.input(event) {
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
                            state.device.borrow_mut().resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            window.resize(**new_inner_size);
                            state.device.borrow_mut().resize(**new_inner_size);                       
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window._window.id() => {
                state.update();
                state.render();
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window._window.request_redraw();
            }
            _ => {}
        }
    });
}