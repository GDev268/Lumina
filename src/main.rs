mod components;
mod data;
mod engine;
mod graphics;

use std::{cell::RefCell, ffi::c_void, rc::Rc};

use ash::vk::{self};
use components::{camera::Camera, shapes::cube::Cube};
use data::{
    buffer::{self, Buffer},
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, PoolConfig},
};
use engine::{
    device::Device,
    swapchain::{self, Swapchain},
    window::Window,
    FrameInfo,
};
use graphics::{
    pipeline::{Pipeline, PipelineConfiguration},
    renderer::{PhysicalRenderer, self},
    shader::Shader,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::components::game_object::{GameObject, GameObjectTrait};

struct Vertex {
    position: glam::Vec2,
    color: glam::Vec3,
}

macro_rules! add {
    ($object:expr, $game_objects:expr) => {{
        let object_clone = Rc::clone(&$object);
        $game_objects.push(object_clone);
    }};
}

fn make_model(vertices: Vec<Vertex>, device: &Device) -> (u32, vk::Buffer) {
    let vertex_count = vertices.len() as u32;

    let buffer_size: vk::DeviceSize =
        (std::mem::size_of::<Vertex>() as vk::DeviceSize) * vertex_count as vk::DeviceSize;

    let (vertex_buffer, vertex_buffer_memory) = device.create_buffer(
        buffer_size,
        vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    );

    let mut data: *mut c_void = std::ptr::null_mut();
    unsafe {
        data = device
            .device()
            .map_memory(
                vertex_buffer_memory,
                0,
                buffer_size,
                vk::MemoryMapFlags::empty(),
            )
            .expect("Failed to map memory on the buffer!");

        std::ptr::copy_nonoverlapping(vertices.as_ptr(), data as *mut Vertex, buffer_size as usize);

        device.device().unmap_memory(vertex_buffer_memory);
    }

    return (vertex_count, vertex_buffer);
}

fn bind(command_buffer:vk::CommandBuffer,vertex_buffer:vk::Buffer,device: &Device){
    let buffers = [vertex_buffer];
    let offset = [0];
    
    unsafe{
        device.device().cmd_bind_vertex_buffers(command_buffer, 0, &buffers, &offset);
    }
}

fn draw(command_buffer:vk::CommandBuffer,device: &Device,vertex_count:u32){
    unsafe{
        device.device().cmd_draw(command_buffer, vertex_count, 1, 0, 0);
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let mut window = Window::new(&event_loop, "Hello Vulkan!", 800, 640);
    let device = Device::new(&window);
    let mut renderer = PhysicalRenderer::new(&window,&device,None);

    let mut command_buffers: Vec<vk::CommandBuffer> = Vec::new();

    let vertices: Vec<Vertex> = vec![
        Vertex {
            position: glam::vec2(0.0, -0.5),
            color: glam::vec3(1.0, 0.0, 0.0),
        },
        Vertex {
            position: glam::vec2(0.5, 0.5),
            color: glam::vec3(0.0, 1.0, 0.0),
        },
        Vertex {
            position: glam::vec2(-0.5, 0.5),
            color: glam::vec3(0.0, 0.0, 1.0),
        },
    ];
    let shader = Shader::new(
        &device,
        "shaders/simple_shader.vert.spv",
        "shaders/simple_shader.frag.spv",
    );

    let (count, buffer) = make_model(vertices, &device);

    renderer.create_pipeline_layout(&device);
    renderer.create_pipeline(renderer.get_swapchain_renderpass(), &shader, &device);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        let command_buffer = renderer.begin_frame(&device, &window);


        renderer.begin_swapchain_renderpass(command_buffer, &device);
        renderer.render_game_objects(&device,command_buffer);
        bind(command_buffer, buffer, &device);
        draw(command_buffer, &device, count);
        renderer.end_swapchain_renderpass(command_buffer, &device);
        renderer.end_frame(&device, &mut window);



        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window._window.id() => control_flow.set_exit(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_size) => {
                    Window::framebuffer_resize_callback(
                        &mut window,
                        new_size.width,
                        new_size.height,
                    );
                }
                _ => (),
            },

            Event::MainEventsCleared => {
                let _ = &window._window.request_redraw();
            }
            _ => (),
        }
    });
}