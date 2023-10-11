use std::{fs::File, io::Read, collections::HashMap, ops::Deref};

use ash::vk;

use revier_core::device::Device;
use glsl_parser::parser::Parser;

pub struct Shader {
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
    pub vert_path: String,
    pub frag_path: String,
    pub shader_structs:HashMap<String,Vec<(String,String)>>, 
    pub push_values:HashMap<String,Vec<(String,String)>>,
    pub descriptor_values:HashMap<String,Vec<(String,String)>>,
    pub push_fields:HashMap<String,vk::PushConstantRange>,
    pub descriptor_fields:HashMap<String,vk::DescriptorSetLayout>
}

impl Shader {
    pub fn new(device: &Device, vert_file_path: &str, frag_file_path: &str) -> Self {
        let mut shader_structs:HashMap<String,Vec<(String,String)>> = HashMap::new(); 
        let mut push_values:HashMap<String,Vec<(String,String)>> = HashMap::new();
        let mut descriptor_values:HashMap<String,Vec<(String,String)>> = HashMap::new();
        let mut push_fields:HashMap<String,vk::PushConstantRange> = HashMap::new();

        let mut parser = Parser::new(); 

        parser.parse_shader(vert_file_path,frag_file_path);

        for (name,values) in parser.vert_structs.iter(){
            shader_structs.insert("VERT-".to_string() + name, values.clone());
        }


        for (name,values) in parser.vert_push_constants.iter(){
            push_values.insert(name.to_owned(), values.clone());
            
        }

        for (name,values) in parser.vert_descriptors.iter(){
            descriptor_values.insert(name.to_owned(), values.clone());
        }

        println!("{:?}",shader_structs);
        println!("{:?}",push_values);
        println!("{:?}",descriptor_values);

        for (name,values) in parser.frag_structs.iter(){
            shader_structs.insert("FRAG-".to_string() + name, values.clone());
        }

        for (name,values) in parser.frag_push_constants{
            if push_values.contains_key(&name) && &values == push_values.get(&name).unwrap(){
                
            }
        }

        return Self {
            vert_module: Shader::create_shader_module(Shader::read_file(String::from(vert_file_path.to_owned() + &".spv".to_owned())), device),
            frag_module: Shader::create_shader_module(Shader::read_file(String::from(frag_file_path.to_owned() + &".spv".to_owned())), device),
            vert_path: String::from(vert_file_path),
            frag_path: String::from(frag_file_path),
            shader_structs,
            push_values,
            descriptor_values,
            push_fields: HashMap::new(),
            descriptor_fields: HashMap::new()
        };
    }

    pub fn read_file(file_path: String) -> Vec<u8> {
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

//PLAN HAVING THE GLSL_TYPES ENUM HAVING THE TYPE NAME WITH THEIR RESPECTIVE RUST TYPE EQUAL
//THEN PARSING FROM STRING TO ENUM AND THEN PUTTING IN AN POOL WITHIN THE SHADER WITH THE
//DESCRIPTOR/PUSH_CONSTANT NAME AND THEN ADDING THE LINE WHERE FROM THE VARIABLE THAT THIER TYPE
//WAS PARSED AND THEN VERYFING IF THE TYPE IS WITHIN THE PUSH/DESCRIPTOR AND IN THE RIGHT POSITION
//AND IF NOT PASS AN WARNING MESSAGE WITH AN DEFAULT VALUE FROM THAT TYPE

//UPDATES ON THE CREATION OF THE GLSL PARSER 22-09-2023:
//NOW THE GLSL_PARSER WILL PARSER THE INPUTS FROM VULKAN (DESCRIPTORS/PUSH_CONSTANTS) AND THEIR
//NESTED STRUCTS WOULD BE READ UNTIL IT REACHES IT'S SIMPLICITY INCLUDING OF THE TYPES AND THE
//STRUCTS WOULD BE SAVED IN AN HASHMAP
//THE VULKAN INPUTS AND STRUCTS WOULD BE AN HASHMAP WITH AN STRING OF THE NAME AN VECTOR OF STRINGS THAT WOULD 
//DIRECT TO ANOTHER HASHMAP THAT WOULD CONTAIN THE ENUM VALUE
