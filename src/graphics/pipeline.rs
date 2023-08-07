


use crate::engine::device::Device;
use ash::vk::{self};




pub struct PipelineConfiguration {
    pub binding_descriptions: Vec<vk::VertexInputBindingDescription>,
    pub attribute_descriptions: Vec<vk::VertexInputAttributeDescription>,
    pub viewport_info: vk::PipelineViewportStateCreateInfo,
    pub input_assembly_info: vk::PipelineInputAssemblyStateCreateInfo,
    pub rasterization_info: vk::PipelineRasterizationStateCreateInfo,
    pub multisample_info: vk::PipelineMultisampleStateCreateInfo,
    pub color_blend_attachment: vk::PipelineColorBlendAttachmentState,
    pub color_blend_info: vk::PipelineColorBlendStateCreateInfo,
    pub depth_stencil_info: vk::PipelineDepthStencilStateCreateInfo,
    pub dynamic_state_info: vk::PipelineDynamicStateCreateInfo,
    pub pipeline_layout: Option<vk::PipelineLayout>,
    pub renderpass: Option<vk::RenderPass>,
    pub subpass: u32,
}

impl PipelineConfiguration {
    pub fn default() -> Self {
        let mut input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::default();
        input_assembly_info.s_type = vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
        input_assembly_info.topology = vk::PrimitiveTopology::TRIANGLE_LIST;
        input_assembly_info.primitive_restart_enable = vk::FALSE;

        let mut viewport_info = vk::PipelineViewportStateCreateInfo::default();
        viewport_info.s_type = vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO;
        viewport_info.viewport_count = 1;
        viewport_info.p_viewports = std::ptr::null();
        viewport_info.scissor_count = 1;
        viewport_info.p_scissors = std::ptr::null();

        let mut rasterization_info = vk::PipelineRasterizationStateCreateInfo::default();
        rasterization_info.s_type = vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
        rasterization_info.depth_clamp_enable = vk::FALSE;
        rasterization_info.rasterizer_discard_enable = vk::FALSE;
        rasterization_info.polygon_mode = vk::PolygonMode::FILL;
        rasterization_info.line_width = 1.0;
        rasterization_info.cull_mode = vk::CullModeFlags::NONE;
        rasterization_info.front_face = vk::FrontFace::CLOCKWISE;
        rasterization_info.depth_bias_enable = vk::FALSE;
        rasterization_info.depth_bias_constant_factor = 0.0;
        rasterization_info.depth_bias_clamp = 0.0;
        rasterization_info.depth_bias_slope_factor = 0.0;

        let mut multisample_info = vk::PipelineMultisampleStateCreateInfo::default();
        multisample_info.s_type = vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
        multisample_info.sample_shading_enable = vk::FALSE;
        multisample_info.rasterization_samples = vk::SampleCountFlags::TYPE_1;
        multisample_info.min_sample_shading = 1.0;
        multisample_info.p_sample_mask = std::ptr::null();
        multisample_info.alpha_to_coverage_enable = vk::FALSE;
        multisample_info.alpha_to_one_enable = vk::FALSE;

        let mut color_blend_attachment = vk::PipelineColorBlendAttachmentState::default();
        color_blend_attachment.color_write_mask = vk::ColorComponentFlags::R
            | vk::ColorComponentFlags::G
            | vk::ColorComponentFlags::B
            | vk::ColorComponentFlags::A;
        color_blend_attachment.blend_enable = vk::FALSE;
        color_blend_attachment.src_color_blend_factor = vk::BlendFactor::ONE;
        color_blend_attachment.dst_color_blend_factor = vk::BlendFactor::ZERO;
        color_blend_attachment.color_blend_op = vk::BlendOp::ADD;
        color_blend_attachment.src_alpha_blend_factor = vk::BlendFactor::ONE;
        color_blend_attachment.dst_alpha_blend_factor = vk::BlendFactor::ZERO;
        color_blend_attachment.alpha_blend_op = vk::BlendOp::ADD;

        let mut color_blend_info = vk::PipelineColorBlendStateCreateInfo::default();
        color_blend_info.s_type = vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
        color_blend_info.logic_op_enable = vk::FALSE;
        color_blend_info.logic_op = vk::LogicOp::COPY;
        color_blend_info.attachment_count = 1;
        color_blend_info.p_attachments = &color_blend_attachment;
        color_blend_info.blend_constants[0] = 0.0;
        color_blend_info.blend_constants[1] = 0.0;
        color_blend_info.blend_constants[2] = 0.0;
        color_blend_info.blend_constants[3] = 0.0;

        let mut depth_stencil_info = vk::PipelineDepthStencilStateCreateInfo::default();
        depth_stencil_info.s_type = vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
        depth_stencil_info.depth_test_enable = vk::TRUE;
        depth_stencil_info.depth_write_enable = vk::TRUE;
        depth_stencil_info.depth_compare_op = vk::CompareOp::LESS;
        depth_stencil_info.depth_bounds_test_enable = vk::FALSE;
        depth_stencil_info.min_depth_bounds = 0.0;
        depth_stencil_info.max_depth_bounds = 1.0;
        depth_stencil_info.stencil_test_enable = vk::FALSE;

        let dynamic_state_enables = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let mut dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default();
        dynamic_state_info.s_type = vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO;
        dynamic_state_info.p_dynamic_states = dynamic_state_enables.as_ptr();
        dynamic_state_info.dynamic_state_count = dynamic_state_enables.len() as u32;
        dynamic_state_info.flags = vk::PipelineDynamicStateCreateFlags::empty();

        Self {
            binding_descriptions: Vec::new(),
            attribute_descriptions: Vec::new(),
            viewport_info: viewport_info,
            input_assembly_info: input_assembly_info,
            rasterization_info: rasterization_info,
            multisample_info: multisample_info,
            color_blend_attachment: color_blend_attachment,
            color_blend_info: color_blend_info,
            depth_stencil_info: depth_stencil_info,
            dynamic_state_info: dynamic_state_info,
            pipeline_layout: None,
            renderpass: None,
            subpass: 0,
        }
    }

    pub fn enable_alpha_blending(&mut self) {
        self.color_blend_attachment.blend_enable = vk::TRUE;
        self.color_blend_attachment.color_write_mask = vk::ColorComponentFlags::R
            | vk::ColorComponentFlags::G
            | vk::ColorComponentFlags::B
            | vk::ColorComponentFlags::A;
        self.color_blend_attachment.src_color_blend_factor = vk::BlendFactor::SRC_ALPHA;
        self.color_blend_attachment.dst_color_blend_factor = vk::BlendFactor::ONE_MINUS_SRC_ALPHA;
        self.color_blend_attachment.color_blend_op = vk::BlendOp::ADD;
        self.color_blend_attachment.src_alpha_blend_factor = vk::BlendFactor::ONE;
        self.color_blend_attachment.dst_alpha_blend_factor = vk::BlendFactor::ZERO;
        self.color_blend_attachment.alpha_blend_op = vk::BlendOp::ADD;
    }
}

pub struct Pipeline {
    pub graphics_pipeline: Option<vk::Pipeline>,
    pub vert_shader_module: Option<vk::ShaderModule>,
    pub frag_shader_module: Option<vk::ShaderModule>,
}

impl Pipeline {
    pub fn new(
        device: &Device,
        _vert_shader: &vk::ShaderModule,
        _frag_shader: &vk::ShaderModule,
        pipeline_config: PipelineConfiguration,
    ) -> Self {
        assert!(
            pipeline_config.pipeline_layout.is_none() == false,
            "Cannot create Graphics Pipeline: pipeline_layout doesn't exist!"
        );
        assert!(
            pipeline_config.renderpass.is_none() == false,
            "Cannot create Graphics Pipeline: renderpass doesn't exist!"
        );

        let mut pipeline = Pipeline::default();

        let mut shader_stages: [vk::PipelineShaderStageCreateInfo; 2] = [
            vk::PipelineShaderStageCreateInfo::default(),
            vk::PipelineShaderStageCreateInfo::default(),
        ];

        shader_stages[0].s_type = vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO;
        shader_stages[0].stage = vk::ShaderStageFlags::VERTEX;
        shader_stages[0].module = pipeline.vert_shader_module.unwrap();
        shader_stages[0].p_name = "main".as_ptr() as *const i8;
        shader_stages[0].flags = vk::PipelineShaderStageCreateFlags::empty();
        shader_stages[0].p_next = std::ptr::null();
        shader_stages[0].p_specialization_info = std::ptr::null();

        shader_stages[0].s_type = vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO;
        shader_stages[0].stage = vk::ShaderStageFlags::FRAGMENT;
        shader_stages[0].module = pipeline.frag_shader_module.unwrap();
        shader_stages[0].p_name = "main".as_ptr() as *const i8;
        shader_stages[0].flags = vk::PipelineShaderStageCreateFlags::empty();
        shader_stages[0].p_next = std::ptr::null();
        shader_stages[0].p_specialization_info = std::ptr::null();

        let vertex_input_info: vk::PipelineVertexInputStateCreateInfo =
            vk::PipelineVertexInputStateCreateInfo {
                flags: vk::PipelineVertexInputStateCreateFlags::empty(),
                s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                vertex_attribute_description_count: 0,
                vertex_binding_description_count: 0,
                p_vertex_attribute_descriptions: std::ptr::null(),
                p_vertex_binding_descriptions: std::ptr::null(),
                p_next: std::ptr::null(),
            };

        let create_info: vk::GraphicsPipelineCreateInfo = vk::GraphicsPipelineCreateInfo {
            flags: vk::PipelineCreateFlags::empty(),
            s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: &vertex_input_info,
            p_input_assembly_state: &pipeline_config.input_assembly_info,
            p_viewport_state: &pipeline_config.viewport_info,
            p_rasterization_state: &pipeline_config.rasterization_info,
            p_multisample_state: &pipeline_config.multisample_info,
            p_color_blend_state: &pipeline_config.color_blend_info,
            p_depth_stencil_state: &pipeline_config.depth_stencil_info,
            p_dynamic_state: &pipeline_config.dynamic_state_info,
            layout: pipeline_config.pipeline_layout.unwrap(),
            render_pass: pipeline_config.renderpass.unwrap(),
            subpass: pipeline_config.subpass,
            base_pipeline_index: -1,
            base_pipeline_handle: vk::Pipeline::null(),
            p_tessellation_state: std::ptr::null(),
            p_next: std::ptr::null(),
        };

        unsafe {
            let graphics_pipelines = device
                .device()
                .create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None)
                .expect("Failed to create graphics pipeline!");
            pipeline.graphics_pipeline = Some(graphics_pipelines[0]);
        }

        return pipeline;
    }

    pub fn default() -> Self {
        return Self {
            graphics_pipeline: None,
            vert_shader_module: None,
            frag_shader_module: None,
        };
    }

    pub fn bind(&self, device: &Device, command_buffer: vk::CommandBuffer) {
        unsafe {
            device.device().cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.graphics_pipeline.unwrap(),
            );
        }
    }
}
