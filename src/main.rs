mod components;
mod data;
mod engine;
mod graphics;

use std::{cell::RefCell, ffi::c_void, rc::Rc};

use ash::vk::{self};
use components::{camera::Camera, shapes::cube::{Cube, PushConstantData}};
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
    shader::Shader, mesh::{Vertex, Mesh},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::components::game_object::{GameObject, GameObjectTrait};


macro_rules! add {
    ($object:expr, $game_objects:expr) => {{
        let object_clone = Rc::clone(&$object);
        $game_objects.push(object_clone);
    }};
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

   
    let shader = Shader::new(
        &device,
        "shaders/simple_shader.vert.spv",
        "shaders/simple_shader.frag.spv",
    );
    
    let mut model = Cube::new(&device);
    
    renderer.create_pipeline_layout(&device);
    renderer.create_pipeline(renderer.get_swapchain_renderpass(), &shader, &device);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        let command_buffer = renderer.begin_frame(&device, &window);


        renderer.begin_swapchain_renderpass(command_buffer, &device);
        renderer.render_game_objects(&device,command_buffer);

        let push: PushConstantData = PushConstantData {
            model_matrix: model.game_object().transform.get_mat4(),
            normal_matrix: model
                .game_object()
                .transform
                .get_normal_matrix(),
        };

        let push_bytes: &[u8] = unsafe {
            let struct_ptr = &push as *const _ as *const u8;
            std::slice::from_raw_parts(struct_ptr, std::mem::size_of::<PushConstantData>())
        };

        unsafe {
            device.device().cmd_push_constants(
                command_buffer,
                renderer.pipeline_layout,
                vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                0,
                push_bytes,
            );

        }

        model.test_render(command_buffer, &device);
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

