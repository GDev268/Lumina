use crate::engine::{
    device::Device,
    swapchain::{self, Swapchain, MAX_FRAMES_IN_FLIGHT},
    window::Window,
};
use ash::vk;
use glfw::Glfw;

pub struct Renderer {
    swapchain: Swapchain,
    pub command_buffers: Vec<vk::CommandBuffer>,
    current_image_index: u32,
    current_frame_index: i32,
    is_frame_started: bool,
}

impl Renderer {
    pub fn new(
        window: &Window,
        device: &Device,
        glfw: &mut Glfw,
        swapchain: Option<&Swapchain>,
    ) -> Self {
        let swapchain = Renderer::recreate_swapchain(window, device, glfw, swapchain);
        let command_buffers = Renderer::create_command_buffers(device);

        return Self {
            swapchain: swapchain,
            command_buffers: command_buffers,
            current_image_index: 0,
            current_frame_index: 0,
            is_frame_started: false,
        };
    }

    pub fn get_swapchain_renderpass(&self) -> vk::RenderPass {
        return self.swapchain.get_renderpass();
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        return self.get_aspect_ratio();
    }

    pub fn is_frame_in_progress(&self) -> bool {
        return self.is_frame_started;
    }

    pub fn get_current_command_buffer(&self) -> vk::CommandBuffer {
        assert!(
            self.is_frame_started,
            "Cannot get command buffer when frame not in progress"
        );

        return self.command_buffers[self.current_frame_index as usize];
    }

    pub fn get_frame_index(&self) -> i32 {
        assert!(
            self.is_frame_started,
            "Cannot get frame index when frame not in progress"
        );

        return self.current_frame_index;
    }

    pub fn begin_frame(&self) -> vk::CommandBuffer {
        return vk::CommandBuffer::null();
    }

    pub fn end_frame(&mut self, device: &Device, window: &mut Window, glfw: &mut Glfw) {
        assert!(
            self.is_frame_started,
            "Cannot end frame when frame not in progress"
        );

        let command_buffer = self.get_current_command_buffer();

        unsafe {
            device
                .device()
                .end_command_buffer(command_buffer)
                .expect("Failed to record command buffer!");
        }

        let result: bool =
            self.swapchain
                .submit_command_buffers(device, command_buffer, self.current_image_index);

        if result || window.was_window_resized() {
            window.reset_window_resized_flag();

            let swapchain = &self.swapchain;
            self.swapchain = Renderer::recreate_swapchain(window, device, glfw, Some(swapchain))
        }
    }

    pub fn begin_swapchain_renderpass(&self, command_buffer: vk::CommandBuffer,device: &Device) {
        assert!(
            self.is_frame_started,
            "Cannot begin swapchain renderpass when frame not in progress"
        );

        assert!(
            command_buffer == self.get_current_command_buffer(),
            "Can't begin render pass on command buffer from a differnt frame"
        );

        let mut clear_values: [vk::ClearValue; 2] =
            [vk::ClearValue::default(), vk::ClearValue::default()];

        clear_values[0].color = vk::ClearColorValue {
            float32: [0.0, 0.0, 0.0, 1.0],
        };
        clear_values[1].depth_stencil = vk::ClearDepthStencilValue {
            depth: 1.0,
            stencil: 0,
        };

        let renderpass_info = vk::RenderPassBeginInfo{
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            render_pass: self.swapchain.get_renderpass(),
            framebuffer: self.swapchain.get_framebuffer(self.current_image_index as usize),
            render_area: vk::Rect2D { offset: vk::Offset2D { x:0, y: 0 }, extent: self.swapchain.get_swapchain_extent() },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr()
        };

        unsafe{
            device.device().cmd_begin_render_pass(command_buffer, &renderpass_info, vk::SubpassContents::INLINE)
        }

        let viewport: vk::Viewport = vk::Viewport{
            x: 0.0,
            y: 0.0,
            width: self.swapchain.get_swapchain_extent().width as f32,
            height: self.swapchain.get_swapchain_extent().height as f32,
            min_depth: 0.0,
            max_depth: 1.0
        };

        let scissor: vk::Rect2D = vk::Rect2D{
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.swapchain.get_swapchain_extent()
        };

        unsafe{
            device.device().cmd_set_viewport(command_buffer, 0, &[viewport]);
            device.device().cmd_set_scissor(command_buffer, 0, &[scissor]);
        }


    }

    pub fn end_swapchain_renderpass(&self, command_buffer: vk::CommandBuffer,device: &Device) {
        assert!(
            self.is_frame_started,
            "Cannot end swapchain renderpass when frame not in progress"
        );

        assert!(
            command_buffer == self.get_current_command_buffer(),
            "Can't end render pass on command buffer from a differnt frame"
        );

        unsafe{
            device.device().cmd_end_render_pass(command_buffer);
        }
    }

    fn create_command_buffers(device: &Device) -> Vec<vk::CommandBuffer> {
        let alloc_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            level: vk::CommandBufferLevel::PRIMARY,
            command_pool: device.get_command_pool(),
            command_buffer_count: MAX_FRAMES_IN_FLIGHT as u32,
        };

        let command_buffers = unsafe {
            device
                .device()
                .allocate_command_buffers(&alloc_info)
                .expect("Failed to allocate command buffers!")
        };

        return command_buffers;
    }

    fn free_command_buffers(&self,device: &Device) {
        unsafe{
            device.device().free_command_buffers(device.get_command_pool(), &self.command_buffers);
        }
    }

    fn recreate_swapchain(
        window: &Window,
        device: &Device,
        glfw: &mut Glfw,
        swapchain: Option<&Swapchain>,
    ) -> Swapchain {
        let mut extent: vk::Extent2D = window.get_extent();

        while extent.width == 0 || extent.height == 0 {
            extent = window.get_extent();
            glfw.wait_events();
        }

        unsafe {
            device
                .device()
                .device_wait_idle()
                .expect("Failed to make device idle!");
        }

        let new_swapchain = if swapchain.is_none() {
            Swapchain::new(device, window.get_extent())
        } else {
            Swapchain::renew(device, window.get_extent(), swapchain.unwrap())
        };

        return new_swapchain;
    }
}
