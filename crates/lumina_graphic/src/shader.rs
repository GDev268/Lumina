use std::{fs::File, io::Read, rc::Rc};

use ash::vk;

use glsl_parser::parser::Parser;
use lumina_core::device::Device;
use lumina_data::{descriptor::PoolConfig, descriptor_manager::DescriptorManager};

pub struct Shader {
    pub descriptor_manager: DescriptorManager,
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
}

impl Shader {
    pub fn new(device: Rc<Device>, vert_file_path: &str, frag_file_path: &str) -> Self {
        let mut parser = Parser::new();

        parser.parse_shader("shaders/default_shader.vert", "shaders/default_shader.frag");

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
            descriptor_manager,
            vert_module: Shader::create_shader_module(Shader::read_file(vert_file_path), &device),
            frag_module: Shader::create_shader_module(Shader::read_file(frag_file_path), &device),
        };
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
