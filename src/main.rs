mod buffer;
mod device;
mod game;
mod old_pipeline;
mod pipeline;
mod swapchain;
mod window;

use crate::pipeline::{Pipeline, PipelineConfiguration};
use crate::window::Window;
use crate::{device::Device, swapchain::Swapchain};
use ash::vk;
use buffer::Buffer;
use glfw::{self};
use glfw::{Action, Context, Key};
use old_pipeline::{OLD_Pipeline, OLD_PipelineConfiguration};
use simple_logger::SimpleLogger;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let window = Window::new(&mut glfw, "Hello Vulkan!", 800, 640);
    let device = Device::new(&window, &glfw);
    let swapchain = Swapchain::new(&device, window.getExtent());

    let mut command_buffers: Vec<vk::CommandBuffer> = Vec::new();

    let pipeline_layout_info: vk::PipelineLayoutCreateInfo =
        vk::PipelineLayoutCreateInfo::default();

    let pipeline_layout: vk::PipelineLayout = unsafe {
        device
            .device()
            .create_pipeline_layout(&pipeline_layout_info, None)
            .expect("Failed to create pipeline layout!")
    };

    let mut pipeline_config =
        OLD_PipelineConfiguration::default(window.getExtent().width, window.getExtent().height);

    pipeline_config.renderpass = Some(swapchain.get_renderpass());
    pipeline_config.pipeline_layout = Some(pipeline_layout);

    let pipeline = OLD_Pipeline::new(
        &device,
        "shaders/simple_shader.vert.spv",
        "shaders/simple_shader.frag.spv",
        pipeline_config,
    );

    command_buffers.resize(swapchain.image_count(), vk::CommandBuffer::default());

    let allocate_info: vk::CommandBufferAllocateInfo = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        command_pool: device.get_command_pool(),
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: command_buffers.len() as u32,
    };

    unsafe {
        command_buffers = device
            .device()
            .allocate_command_buffers(&allocate_info)
            .expect("Failed to allocate command buffers!");
    }

    for i in 0..command_buffers.len() {
        let begin_info = vk::CommandBufferBeginInfo{
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: std::ptr::null()
        };

        
    }
}
