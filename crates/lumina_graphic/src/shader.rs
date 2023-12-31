use std::{fs::File, io::Read, rc::Rc};

use ash::vk;

use glsl_parser::parser::Parser;
use lumina_core::device::Device;
use lumina_data::{descriptor::PoolConfig, descriptor_manager::DescriptorManager};
use lumina_object::game_object::Component;

use crate::pipeline::{Pipeline, PipelineConfiguration};

pub struct PushConstantData {
    pub model_matrix: glam::Mat4,
    pub normal_matrix: glam::Mat4,
}

pub struct Shader {
    device: Rc<Device>,
    pub descriptor_manager: DescriptorManager,
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
    pub pipeline_layout: Option<vk::PipelineLayout>,
    pub pipeline: Option<Pipeline>,
}

impl Shader {
    pub fn new(device: Rc<Device>, vert_file_path: &str, frag_file_path: &str) -> Self {
        let mut parser = Parser::new();

        parser.parse_shader(vert_file_path, frag_file_path);

        let mut pool_config = PoolConfig::new();
        pool_config.set_max_sets(3 * lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32);
        pool_config.add_pool_size(
            vk::DescriptorType::UNIFORM_BUFFER,
            parser.descriptor_data.len() as u32
                * lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32,
        );
        pool_config.add_pool_size(
            vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32,
        );

        let mut descriptor_manager =
            DescriptorManager::new(Rc::clone(&device), pool_config.build(&device));
        for (name, values) in parser.descriptor_data.iter() {
            descriptor_manager.add_new_descriptor(
                name.to_owned(),
                values.binding,
                values.is_uniform,
            );
            descriptor_manager.build_descriptor(name, values.size as u64);
        }

        descriptor_manager.print_weege();
        descriptor_manager.preload_we();

        return Self {
            device: Rc::clone(&device),
            descriptor_manager,
            vert_module: Shader::create_shader_module(
                Shader::read_file(&(vert_file_path.to_string() + ".spv")),
                &device,
            ),
            frag_module: Shader::create_shader_module(
                Shader::read_file(&(frag_file_path.to_string() + ".spv")),
                &device,
            ),
            pipeline: None,
            pipeline_layout: None,
        };
    }

    pub fn create_pipeline_layout(&mut self, contains_push_constants: bool) {
        let push_constant_range: vk::PushConstantRange = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
            offset: 0,
            size: std::mem::size_of::<PushConstantData>() as u32,
        };

        let descriptor_set_layouts = vec![self
            .descriptor_manager
            .get_descriptor_layout()
            .get_descriptor_set_layout()];

        let pipeline_layout_info: vk::PipelineLayoutCreateInfo = if contains_push_constants {
            let push_constant_range: vk::PushConstantRange = vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                offset: 0,
                size: std::mem::size_of::<PushConstantData>() as u32,
            };

            vk::PipelineLayoutCreateInfo {
                s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: vk::PipelineLayoutCreateFlags::empty(),
                set_layout_count: descriptor_set_layouts.len() as u32,
                p_set_layouts: descriptor_set_layouts.as_ptr(),
                push_constant_range_count: 1,
                p_push_constant_ranges: &push_constant_range,
            }
        } else {
            vk::PipelineLayoutCreateInfo {
                s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: vk::PipelineLayoutCreateFlags::empty(),
                set_layout_count: descriptor_set_layouts.len() as u32,
                p_set_layouts: descriptor_set_layouts.as_ptr(),
                push_constant_range_count: 0,
                p_push_constant_ranges: std::ptr::null(),
            }
        };

        unsafe {
            self.pipeline_layout = Some(
                self.device
                    .device()
                    .create_pipeline_layout(&pipeline_layout_info, None)
                    .expect("Failed to create pipeline layout!"),
            );
        }
    }

    pub fn create_pipeline(&mut self, render_pass: vk::RenderPass,mut pipeline_config:PipelineConfiguration) {
        pipeline_config.renderpass = Some(render_pass);
        pipeline_config.pipeline_layout = self.pipeline_layout;

        self.pipeline = Some(Pipeline::new(
            &self.device,
            self.vert_module,
            self.frag_module,
            &mut pipeline_config,
        ));
    }

    pub fn read_file(file_path: &str) -> Vec<u8> {
        let file = File::open(file_path).expect("Failed to open shader file");

        return file.bytes().filter_map(|byte| byte.ok()).collect();
    }

    pub fn create_shader_module(code: Vec<u8>, device: &Device) -> vk::ShaderModule {
        let create_info: vk::ShaderModuleCreateInfo = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: code.len(),
            p_code: code.as_ptr() as *const u32,
        };

        unsafe {
            return device
                .device()
                .create_shader_module(&create_info, None)
                .expect("Failed to create shader module!");
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.device.device().device_wait_idle().unwrap();
            self.device
                .device()
                .destroy_shader_module(self.vert_module, None);
            self.device
                .device()
                .destroy_shader_module(self.frag_module, None);
            if self.pipeline_layout.is_some() {
                self.device
                    .device()
                    .destroy_pipeline_layout(self.pipeline_layout.unwrap(), None)
            }
        }
    }
}

impl Component for Shader {
    fn max_component_count() -> Option<usize> {
        Some(1)
    }
}

unsafe impl Send for Shader {}
