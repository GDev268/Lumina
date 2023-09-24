//PLANS FOR THE SHADER STRUCT RENOVATION:
//HAVING THE GLSL_TYPES ENUM HAVING THE TYPE NAME WITH THEIR RESPECTIVE RUST TYPE EQUAL
//THEN PARSING FROM STRING TO ENUM AND THEN PUTTING IN AN POOL WITHIN THE SHADER WITH THE
//DESCRIPTOR/PUSH_CONSTANT NAME AND THEN ADDING THE LINE WHERE FROM THE VARIABLE THAT THIER TYPE
//WAS PARSED AND THEN VERYFING IF THE TYPE IS WITHIN THE PUSH/DESCRIPTOR AND IN THE RIGHT POSITION
//AND IF NOT PASS AN WARNING MESSAGE WITH AN DEFAULT VALUE FROM THAT TYPE
//BEING IMPLEMENTED ON AN EVEN SECRET PROJECT THAT IS USED TO EXPERIMENT TECHNIQUES AND CODES

use std::{fs::File, io::Read};

use ash::vk;

use revier_core::device::Device;

pub struct Shader {
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
}

impl Shader {
    pub fn new(device: &Device, vert_file_path: &str, frag_file_path: &str) -> Self {
        return Self {
            vert_module: Shader::create_shader_module(Shader::read_file(vert_file_path), device),
            frag_module: Shader::create_shader_module(Shader::read_file(frag_file_path), device),
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
