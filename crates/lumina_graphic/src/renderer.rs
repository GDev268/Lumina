use std::{collections::HashMap, rc::Rc};

use lumina_core::{
    device::Device,
    swapchain::{self, Swapchain, MAX_FRAMES_IN_FLIGHT},
    window::Window,
};

use lumina_data::descriptor::DescriptorSetLayout;
use lumina_geometry::model::{Model, PushConstantData};
use lumina_object::{game_object::GameObject, transform::Transform};
use lumina_scene::query::Query;

use ash::vk;

use crate::shader::FieldData;

use super::{
    pipeline::{Pipeline, PipelineConfiguration},
    shader::Shader,
    types::{LuminaShaderType, LuminaShaderTypeConverter},
};

pub struct Renderer<'a> {
    swapchain: Swapchain,
    command_buffers: Vec<vk::CommandBuffer>,
    current_image_index: u32,
    current_frame_index: i32,
    is_frame_started: bool,
    pipeline: Option<Pipeline>,
    pipeline_layout: vk::PipelineLayout,
    cur_shader: Option<&'a Shader>,
    cur_cmd: vk::CommandBuffer,
}

impl<'a> Renderer<'a> {
    pub fn new(window: &Window, device: &Device) -> Self {
        let swapchain = Renderer::create_swapchain(window, device, None);
        let command_buffers = Renderer::create_command_buffers(device);

        return Self {
            swapchain,
            command_buffers,
            current_image_index: 0,
            current_frame_index: 0,
            is_frame_started: false,
            pipeline: None,
            pipeline_layout: vk::PipelineLayout::null(),
            cur_shader: None,
            cur_cmd: vk::CommandBuffer::null(),
        };
    }

    fn get_swapchain_renderpass(&self) -> vk::RenderPass {
        return self.swapchain.get_renderpass();
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        return self.swapchain.extent_aspect_ratio();
    }

    fn is_frame_in_progress(&self) -> bool {
        return self.is_frame_started;
    }

    fn get_current_command_buffer(&self) -> vk::CommandBuffer {
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

    /*pub fn activate_shader(&mut self,device:&Device,shader:&'a Shader){
        self.cur_shader = Some(shader);
        self.create_pipeline_layout(device,&self.cur_shader.unwrap().push_fields,&self.cur_shader.unwrap().descriptor_fields);
        self.create_pipeline(device,self.get_swapchain_renderpass());
    }*/

    pub fn begin_frame(&mut self, device: &Device, window: &Window) {
        assert!(
            !self.is_frame_started,
            "Can't begin frame while it is already in progress"
        );

        let result = self.swapchain.acquire_next_image(device);
        if result.is_err() {
            self.recreate_swapchain(device, window);
        };

        self.current_image_index = result.unwrap().0;
        self.is_frame_started = true;

        self.cur_cmd = self.get_current_command_buffer();
        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: vk::CommandBufferUsageFlags::empty(),
            p_inheritance_info: std::ptr::null(),
        };

        unsafe {
            device
                .device()
                .begin_command_buffer(self.cur_cmd, &begin_info)
                .expect("Failed to begin recording command buffer");
        }

        self.begin_swapchain_renderpass(self.cur_cmd, device);
    }

    pub fn end_frame(&mut self, device: &Device, window: &mut Window) {
        self.end_swapchain_renderpass(self.cur_cmd, device);
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

    fn begin_swapchain_renderpass(&self, command_buffer: vk::CommandBuffer, device: &Device) {
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

    fn end_swapchain_renderpass(&self, command_buffer: vk::CommandBuffer, device: &Device) {
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

    pub fn create_pipeline_layout(
        &mut self,
        device: &Device,
        push_fields: &HashMap<String, vk::PushConstantRange>,
        descriptor_set_layout: &DescriptorSetLayout,
    ) {
        let mut push_constant_ranges: Vec<vk::PushConstantRange> = Vec::new();

        for (_, range) in push_fields {
            push_constant_ranges.push(*range);
        }

        let descriptor_set_layouts = vec![descriptor_set_layout.get_descriptor_set_layout()];

        let pipeline_layout_info: vk::PipelineLayoutCreateInfo = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: descriptor_set_layouts.len() as u32,
            p_set_layouts: descriptor_set_layouts.as_ptr(),
            push_constant_range_count: push_constant_ranges.len() as u32,
            p_push_constant_ranges: push_constant_ranges.as_ptr(),
        };

        unsafe {
            self.pipeline_layout = device
                .device()
                .create_pipeline_layout(&pipeline_layout_info, None)
                .expect("Failed to create pipeline layout!");
        }
    }

    pub fn create_pipeline(
        &mut self,
        device: &Device,
        render_pass: vk::RenderPass,
        shader: &Shader,
    ) {
        let mut pipeline_config: PipelineConfiguration = PipelineConfiguration::default();
        pipeline_config.renderpass = Some(render_pass);
        pipeline_config.pipeline_layout = Some(self.pipeline_layout);

        self.pipeline = Some(Pipeline::new(
            device,
            shader.vert_module,
            shader.frag_module,
            &mut pipeline_config,
        ));
    }

    pub fn render_object(
        &mut self,
        device: &Device,
        scene: &mut Query,
        gameobject_id: &GameObject,
    ) {
        for (id, entity) in scene.entities.iter_mut() {
            if id == &gameobject_id.get_id() {
                let shader = entity.get_mut_component::<Shader>().unwrap();

                self.create_pipeline_layout(
                    device,
                    &shader.push_fields,
                    &shader.shader_descriptor_layout,
                );

                self.create_pipeline(device, self.get_swapchain_renderpass(), shader);

                self.pipeline.as_ref().unwrap().bind(device, self.cur_cmd);

                for (name, components) in shader.descriptor_fields.iter_mut() {
                    if !components.is_image {
                        let mut descriptor_bytes: Vec<u8> = Vec::new();

                        for value in shader.descriptor_values.get(name).unwrap().iter() {
                            value.value.to_ne_bytes(&mut descriptor_bytes);
                        }

                        components.buffers[self.get_frame_index() as usize].write_to_buffer(
                            &descriptor_bytes,
                            None,
                            None,
                        );
                        components.buffers[self.get_frame_index() as usize]
                            .flush(None, None, device);
                    }
                }

                unsafe {
                    device.device().cmd_bind_descriptor_sets(
                        self.cur_cmd,
                        vk::PipelineBindPoint::GRAPHICS,
                        self.pipeline_layout,
                        0,
                        &[shader.shader_descriptor_sets[self.get_frame_index() as usize]],
                        &[],
                    );
                }

                for (name, values) in shader.push_values.iter() {
                    let mut push_bytes: Vec<u8> = Vec::new();
                    for value in values.iter() {
                        value.value.to_ne_bytes(&mut push_bytes);
                    }

                    unsafe {
                        device.device().cmd_push_constants(
                            self.cur_cmd,
                            self.pipeline_layout,
                            shader.push_fields.get(name).unwrap().stage_flags,
                            shader
                                .value_sizes
                                .get(&("PUSH-".to_string() + name))
                                .unwrap()
                                .1 as u32,
                            push_bytes.as_ref(),
                        )
                    };
                }

                if entity.has_component::<Model>() {
                    entity
                        .get_mut_component::<Model>()
                        .unwrap()
                        .render(device, self.cur_cmd);
                }
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
        self.swapchain = Renderer::create_swapchain(window, device, None);
        self.command_buffers = Renderer::create_command_buffers(device);
    }

    pub fn cleanup(&mut self, device: &Device) {
        unsafe {
            self.free_command_buffers(device);
            self.command_buffers.clear();
            self.swapchain.cleanup(device);
        }
    }
}
