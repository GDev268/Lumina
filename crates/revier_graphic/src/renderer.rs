use std::rc::Rc;

use revier_core::{
    device::Device,
    swapchain::{self, Swapchain, MAX_FRAMES_IN_FLIGHT},
    window::Window,
};

use revier_geometry::model::{Model, PushConstantData};
use revier_object::transform::Transform;
use revier_scene::{query::Query, FrameInfo};

use ash::vk;

use super::{
    pipeline::{Pipeline, PipelineConfiguration},
    shader::Shader,
};

pub struct PhysicalRenderer {
    pub swapchain: Swapchain,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub current_image_index: u32,
    current_frame_index: i32,
    pub is_frame_started: bool,
    pub pipeline: Option<Pipeline>,
    pub pipeline_layout: vk::PipelineLayout,
    pub shader: Option<Rc<Shader>>,
}

impl PhysicalRenderer {
    pub fn new(
        window: &Window,
        device: &Device,
        shader: Rc<Shader>,
        swapchain: Option<&Swapchain>
    ) -> Self {
        let swapchain = PhysicalRenderer::create_swapchain(window, device, swapchain);
        let command_buffers = PhysicalRenderer::create_command_buffers(device);

        return Self {
            swapchain,
            command_buffers,
            current_image_index: 0,
            current_frame_index: 0,
            is_frame_started: false,
            pipeline: None,
            pipeline_layout: vk::PipelineLayout::null(),
            shader: Some(shader),
        };
    }

    pub fn get_swapchain_renderpass(&self) -> vk::RenderPass {
        return self.swapchain.get_renderpass();
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        return self.swapchain.extent_aspect_ratio();
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

    pub fn begin_frame(&mut self, device: &Device, window: &Window) -> Option<vk::CommandBuffer> {
        assert!(
            !self.is_frame_started,
            "Can't begin frame while it is already in progress"
        );

        let result = self.swapchain.acquire_next_image(device);
        if result.is_err() {
            self.recreate_swapchain(device, window);
            return None;
        };

        self.current_image_index = result.unwrap().0;
        self.is_frame_started = true;

        let command_buffer = self.get_current_command_buffer();
        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: std::ptr::null(),
        };

        unsafe {
            device
                .device()
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin recording command buffer");
        }

        return Some(command_buffer);
    }

    pub fn end_frame(&mut self, device: &Device, window: &mut Window) {
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

        let result: Result<bool, vk::Result> =
            self.swapchain
                .submit_command_buffers(device, command_buffer, self.current_image_index);

        if result.is_err() || window.was_window_resized() {
            window.reset_window_resized_flag();

            self.recreate_swapchain(device, window);
        }

        self.is_frame_started = false;
        self.current_frame_index =
            (self.current_frame_index + 1) % swapchain::MAX_FRAMES_IN_FLIGHT as i32;
    }

    pub fn begin_swapchain_renderpass(&self, command_buffer: vk::CommandBuffer, device: &Device) {
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
            float32: [0.15, 0.15, 0.15, 1.0],
        };
        clear_values[1].depth_stencil = vk::ClearDepthStencilValue {
            depth: 1.0,
            stencil: 0,
        };

        let renderpass_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            render_pass: self.swapchain.get_renderpass(),
            framebuffer: self
                .swapchain
                .get_framebuffer(self.current_image_index as usize),
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.get_swapchain_extent(),
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
        };

        unsafe {
            device.device().cmd_begin_render_pass(
                command_buffer,
                &renderpass_info,
                vk::SubpassContents::INLINE,
            )
        }

        let viewport: vk::Viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: self.swapchain.get_swapchain_extent().width as f32,
            height: self.swapchain.get_swapchain_extent().height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor: vk::Rect2D = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.swapchain.get_swapchain_extent(),
        };

        unsafe {
            device
                .device()
                .cmd_set_viewport(command_buffer, 0, &[viewport]);
            device
                .device()
                .cmd_set_scissor(command_buffer, 0, &[scissor]);
        }
    }

    pub fn end_swapchain_renderpass(&self, command_buffer: vk::CommandBuffer, device: &Device) {
        assert!(
            self.is_frame_started,
            "Cannot end swapchain renderpass when frame not in progress"
        );

        assert!(
            command_buffer == self.get_current_command_buffer(),
            "Can't end render pass on command buffer from a differnt frame"
        );

        unsafe {
            device.device().cmd_end_render_pass(command_buffer);
        }
    }

    pub fn create_pipeline_layout(&mut self, device: &Device) {
        let push_constant_range: vk::PushConstantRange = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
            offset: 0,
            size: std::mem::size_of::<PushConstantData>() as u32,
        };

        let pipeline_layout_info: vk::PipelineLayoutCreateInfo = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 0,
            p_set_layouts: std::ptr::null(),
            push_constant_range_count: 1,
            p_push_constant_ranges: &push_constant_range,
        };

        unsafe {
            self.pipeline_layout = device
                .device()
                .create_pipeline_layout(&pipeline_layout_info, None)
                .expect("Failed to create pipeline layout!");
        }
    }

    pub fn create_pipeline(&mut self, render_pass: vk::RenderPass, device: &Device) {
        let mut pipeline_config: PipelineConfiguration = PipelineConfiguration::default();
        pipeline_config.renderpass = Some(render_pass);
        pipeline_config.pipeline_layout = Some(self.pipeline_layout);

        self.pipeline = Some(Pipeline::new(
            device,
            self.shader.as_ref().unwrap().vert_module,
            self.shader.as_ref().unwrap().frag_module,
            &mut pipeline_config,
        ));
    }

    pub fn render_game_objects(&self, device: &Device, frame_info: &FrameInfo, scene: &mut Query) {
        self.pipeline
            .as_ref()
            .unwrap()
            .bind(device, frame_info.command_buffer);

        /*unsafe {
            device.device().cmd_bind_descriptor_sets(
                frame_info.command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                &[frame_info.global_descriptor_set],
                &[],
            );
        }*/

        for (id, entity) in scene.entities.iter_mut() {
            let push: PushConstantData = if entity.has_component::<Transform>() {
                PushConstantData {
                    model_matrix: frame_info.camera.get_projection()
                        * entity.get_mut_component::<Transform>().unwrap().get_mat4(),
                    normal_matrix: entity
                        .get_mut_component::<Transform>()
                        .unwrap()
                        .get_normal_matrix(),
                }
            } else {
                PushConstantData {
                    model_matrix: glam::Mat4::default(),
                    normal_matrix: glam::Mat4::default(),
                }
            };

            let push_bytes: &[u8] = unsafe {
                let struct_ptr = &push as *const _ as *const u8;
                std::slice::from_raw_parts(struct_ptr, std::mem::size_of::<PushConstantData>())
            };

            unsafe {
                device.device().cmd_push_constants(
                    frame_info.command_buffer,
                    self.pipeline_layout,
                    vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                    0,
                    push_bytes,
                );
            }

            if entity.has_component::<Model>() {
                entity
                    .get_mut_component::<Model>()
                    .unwrap()
                    .render(device, frame_info.command_buffer);
            }
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

    fn free_command_buffers(&self, device: &Device) {
        unsafe {
            device
                .device()
                .free_command_buffers(device.get_command_pool(), &self.command_buffers);
        }
    }

    fn create_swapchain(
        window: &Window,
        device: &Device,
        swapchain: Option<&Swapchain>,
    ) -> Swapchain {
        let mut extent: vk::Extent2D = window.get_extent();

        while extent.width == 0 || extent.height == 0 {
            extent = window.get_extent();
        }

        let new_swapchain = if swapchain.is_none() {
            Swapchain::new(device, window.get_extent())
        } else {
            Swapchain::renew(device, window.get_extent(), swapchain.unwrap())
        };

        return new_swapchain;
    }

    pub fn recreate_swapchain(&mut self, device: &Device, window: &Window) {
        unsafe {
            device
                .device()
                .device_wait_idle()
                .expect("Failed to make device idle!");
        }

        self.cleanup(device);
        self.swapchain = PhysicalRenderer::create_swapchain(window, device, None);
        self.create_pipeline_layout(&device);
        self.create_pipeline(self.get_swapchain_renderpass(), device);
        self.command_buffers = PhysicalRenderer::create_command_buffers(device);
    }

    pub fn cleanup(&mut self, device: &Device) {
        unsafe {
            device
                .device()
                .free_command_buffers(device.get_command_pool(), &self.command_buffers);
            self.command_buffers.clear();
            self.pipeline.as_mut().unwrap().cleanup(device);
            device
                .device()
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.pipeline_layout = vk::PipelineLayout::null();
            self.swapchain.cleanup(device);
        }
    }
}
