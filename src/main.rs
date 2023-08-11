mod components;
mod data;
mod engine;
mod graphics;

use std::ffi::c_void;

use ash::vk::{self};
use components::{game_object, model::Model, shapes::cube::Cube, camera::Camera};
use data::{
    buffer::Buffer,
    descriptor::{DescriptorPool, DescriptorSetLayout, PoolConfig, DescriptorWriter},
};
use engine::{
    device::Device,
    swapchain::{self},
    window::Window, FrameInfo,
};
use graphics::{mesh::Mesh, renderer::PhysicalRenderer, shader::Shader};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::components::game_object::{GameObject, GameObjectTrait};

#[path = "testing/fill.rs"]
mod fill;

struct old_GlobalUBO {
    projection_view: glam::Mat4,
    light_direction: glam::Vec3,
}

impl old_GlobalUBO {
    pub fn default() -> Self {
        return Self {
            projection_view: glam::mat4(
                glam::Vec4::ONE,
                glam::Vec4::ONE,
                glam::Vec4::ONE,
                glam::Vec4::ONE,
            ),
            light_direction: glam::Vec3::normalize(glam::vec3(1.0, -3.0, -1.0)),
        };
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let mut window = Window::new(&event_loop, "Hello Vulkan!", 800, 640);
    let _device = Device::new(&window);
    let mut renderer = PhysicalRenderer::new(&window, &_device, None);

    let mut game_objects: Vec<Box<&dyn GameObjectTrait>> = Vec::new();

    let mut pool_config = PoolConfig::new();
    pool_config.set_max_sets(swapchain::MAX_FRAMES_IN_FLIGHT as u32);
    pool_config.add_pool_size(
        vk::DescriptorType::UNIFORM_BUFFER,
        swapchain::MAX_FRAMES_IN_FLIGHT as u32,
    );

    let global_pool: DescriptorPool = pool_config.build(&_device);

    let mut cube = Cube::new(&_device);
    cube.game_object.transform.translation = glam::vec3(-0.5, 0.5, 2.5);
    cube.game_object.transform.scale = glam::vec3(3.0, 1.5, 3.0);
    game_objects.push(Box::new(&cube));

    let mut cube = Cube::new(&_device);
    cube.game_object.transform.translation = glam::vec3(0.5, 0.5, 2.5);
    cube.game_object.transform.scale = glam::vec3(3.0, 1.5, 3.0);
    game_objects.push(Box::new(&cube));

    let mut ubo_buffers: Vec<Buffer> = Vec::new();

    for i in 0..swapchain::MAX_FRAMES_IN_FLIGHT {
        let mut buffer = Buffer::new(
            &_device,
            std::mem::size_of::<old_GlobalUBO>() as vk::DeviceSize,
            1,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            None,
        );
        buffer.map(&_device, None, None);
        ubo_buffers.push(buffer);
    }

    let global_set_layout = DescriptorSetLayout::build(
        &_device,
        DescriptorSetLayout::add_binding(
            0,
            vk::DescriptorType::UNIFORM_BUFFER,
            vk::ShaderStageFlags::empty(),
            None,
            None,
        ),
    );

    let mut global_descriptor_sets: Vec<vk::DescriptorSet> = Vec::new();
    
    for i in 0..swapchain::MAX_FRAMES_IN_FLIGHT{
        let buffer_info = ubo_buffers[i].descriptor_info(None, None);
        let mut descriptor_writer = DescriptorWriter::new();
        descriptor_writer.write_buffer(0, buffer_info,&global_set_layout);
        let descriptor_set = descriptor_writer.build(&_device, global_set_layout.get_descriptor_set_layout(), &global_pool);
        
        global_descriptor_sets.push(descriptor_set);
        
    }

    let mut camera = Camera::new();

    let viewer_object = GameObject::create_game_object();
    

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        let swapchain_support = _device.get_swapchain_support();

        camera.set_view_yxz(viewer_object.transform.translation, viewer_object.transform.rotation);
    
        let aspect: f32 = renderer.get_aspect_ratio();
        camera.set_perspective_projection(aspect.to_radians(), aspect, 0.1, 10.0);

        let command_buffer = renderer.begin_frame(&_device,&window);
        let frame_index: i32 = renderer.get_frame_index();
            
        let frame_info: FrameInfo<'_> = FrameInfo{
            frame_index,
            frame_time: 0.0,
            command_buffer,
            camera:&camera,
            global_descriptor_set:global_descriptor_sets[frame_index as usize]
        };

        let ubo = old_GlobalUBO{
            projection_view: camera.get_projection() * camera.get_view(),
            light_direction: glam::vec3(0.0, 0.0, -2.0)
        };

        renderer.begin_swapchain_renderpass(command_buffer, &_device);
        renderer.end_swapchain_renderpass(command_buffer, &_device);
        renderer.end_frame(&_device, &mut window);
        renderer.current_image_index = renderer.current_image_index + 1;

        /*match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window._window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                let _ = &window._window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                fill::fill_window(&window._window);
            }
            _ => (),
        }*/


    });
}
