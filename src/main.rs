mod components;
mod data;
mod engine;
mod graphics;

use std::{cell::RefCell, ffi::c_void, ops::{Deref, DerefMut}, rc::Rc, borrow::BorrowMut};

use ash::vk::{self};
use components::{
    camera::Camera,
    model::Model,
    shapes::cube::{Cube, PushConstantData},
};
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
    mesh::{Mesh, Vertex},
    pipeline::{Pipeline, PipelineConfiguration},
    renderer::{self, PhysicalRenderer},
    shader::Shader,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::components::game_object::{GameObject};



macro_rules! add {
    ($object:expr, $game_objects:expr) => {{
        let rc_object = Rc::new(RefCell::new($object));
        $game_objects.push(rc_object.clone());
    }};
}

fn main() {
    let event_loop = EventLoop::new();

    let mut window = Window::new(&event_loop, "Hello Vulkan!", 800, 640);
    let device = Device::new(&window);
    let mut renderer = PhysicalRenderer::new(&window, &device, None);

    let mut command_buffers: Vec<vk::CommandBuffer> = Vec::new();

    /*let mut game_objects: Vec<&dyn GameObjectTrait>;

    let shader = Shader::new(
        &device,
        "shaders/simple_shader.vert.spv",
        "shaders/simple_shader.frag.spv",
    );

    let vertices: Vec<Vertex> = vec![
        // left face (white)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // right face (yellow)
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // top face (orange)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // bottom face (red)
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // nose face (blue)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // tail face (green)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
    ];


    let shader = Shader::new(
        &device,
        "shaders/simple_shader.vert.spv",
        "shaders/simple_shader.frag.spv",
    );

    let mut model = Model::new();
    model.create_mesh_from_array(vertices, Vec::new(), &device);

    model.game_object.transform.translation = glam::vec3(0.0, 0.0, 2.5);
    model.game_object.transform.scale = glam::vec3(1.0, 1.0, 1.0);

    //game_objects.push(&model);

    renderer.create_pipeline_layout(&device);
    renderer.create_pipeline(renderer.get_swapchain_renderpass(), &shader, &device);

    let mut camera = Camera::new();
    let aspect = renderer.get_aspect_ratio();
    camera.set_perspective_projection(50.0_f32.to_radians(), aspect, 0.1, 10.0);*/

    
    event_loop.run(move |event, _, control_flow| {
        /*let command_buffer = renderer.begin_frame(&device, &window);

        let frame_info: FrameInfo<'_> = FrameInfo{
            frame_time: 0.0,
            command_buffer,
            camera: &camera
        };

        renderer.begin_swapchain_renderpass(command_buffer, &device);
        model.game_object.transform.rotation.y =
        (model.game_object().transform.rotation.y + 0.00055) % (std::f32::consts::PI * 2.0);
        model.game_object.transform.rotation.x =
        (model.game_object().transform.rotation.x + 0.00055) % (std::f32::consts::PI * 2.0);
        renderer.render_game_objects(&device, &frame_info,&mut game_objects);


        let push: PushConstantData = PushConstantData {
            model_matrix: camera.get_projection() * model.game_object().transform.get_mat4(),
            normal_matrix: model.game_object().transform.get_normal_matrix(),
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

        model.render(&device, model.game_object(), command_buffer);
        renderer.end_swapchain_renderpass(command_buffer, &device);
        renderer.end_frame(&device, &mut window);*/



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
