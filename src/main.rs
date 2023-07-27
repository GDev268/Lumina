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
    let mut swapchain = Swapchain::new(&device, window.getExtent());

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
        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: std::ptr::null(),
        };

        unsafe {
            device
                .device()
                .begin_command_buffer(command_buffers[i], &begin_info)
                .expect("Failed to begin the command buffer");
        }

        let mut clear_values: [vk::ClearValue; 2] =
            [vk::ClearValue::default(), vk::ClearValue::default()];

        clear_values[0].color = vk::ClearColorValue {
            float32: [0.1, 0.1, 0.1, 1.0],
        };
        clear_values[1].depth_stencil = vk::ClearDepthStencilValue {
            depth: 1.0,
            stencil: 0,
        };

        let render_pass_info: vk::RenderPassBeginInfo = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            render_pass: swapchain.get_renderpass(),
            framebuffer: swapchain.get_framebuffer(i),
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: swapchain.get_swapchain_extent(),
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
        };

        unsafe {
            device.device().cmd_begin_render_pass(
                command_buffers[i],
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );
        }

        pipeline.bind(&device, command_buffers[i]);

        unsafe {
            device.device().cmd_draw(command_buffers[i], 3, 1, 0, 1);

            device.device().cmd_end_render_pass(command_buffers[i]);

            device
                .device()
                .end_command_buffer(command_buffers[i])
                .expect("Failed to record command buffer!");
        }
    }

    while !window._window.should_close() {
        glfw.poll_events();

        let (image_index,_) = swapchain.acquire_next_image(&device);

        swapchain.submit_command_buffers(&device, command_buffers[image_index as usize], image_index);
    }
}
