use std::{fs::File, io::Read, collections::HashMap, ops::Deref, any::Any};

use ash::vk;

use revier_core::device::Device;
use glsl_parser::parser::Parser;
use revier_data::descriptor::DescriptorSetLayout;

#[derive(Debug)]
pub struct FieldData{
    name:String,
    data_type:String,
    value:Box<dyn Any> 
}


pub struct Shader {
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
    pub vert_path: String,
    pub frag_path: String,
    pub shader_structs:HashMap<String,Vec<(String,String)>>, 
    pub push_values:HashMap<String,Vec<FieldData>>,
    pub descriptor_values:HashMap<String,Vec<FieldData>>,
    pub push_fields:HashMap<String,vk::PushConstantRange>,
    pub descriptor_fields:HashMap<String,DescriptorSetLayout>
}

impl Shader {
    pub fn new(device: &Device, vert_file_path: &str, frag_file_path: &str) -> Self {
        let mut shader_structs:HashMap<String,Vec<(String,String)>> = HashMap::new(); 
        let mut push_returns:HashMap<String,Vec<(String,String)>> = HashMap::new();
        let mut descriptor_returns:HashMap<String,Vec<(String,String)>> = HashMap::new();
        let mut push_fields:HashMap<String,vk::PushConstantRange> = HashMap::new();
        let mut descriptor_fields:HashMap<String,DescriptorSetLayout> = HashMap::new();

        let mut parser = Parser::new(); 

        parser.parse_shader(vert_file_path,frag_file_path);

        for (name,values) in parser.vert_structs.iter(){
            shader_structs.insert("VERT-".to_string() + name, values.clone());
        }

        for (name,values) in parser.vert_push_constants.iter(){
            push_returns.insert(name.to_owned(), values.clone());
            let mut max_value = 0;

            for value in values{
                max_value += parser.convert_to_size(&value.0);
            }
            
            push_fields.insert(name.to_owned(), vk::PushConstantRange { stage_flags: vk::ShaderStageFlags::VERTEX, offset: 0, size: max_value as u32 });
        }


        for (name,values) in parser.vert_descriptors.iter(){
            descriptor_returns.insert(name.to_owned(), values.clone());

            let mut max_value = 0;

            for value in values{
                max_value += parser.convert_to_size(&value.0);
            }
           
            if !values.iter().any(|string| string.0.contains("sampler")) {
                let set_layout = DescriptorSetLayout::build(
                    device,
                    DescriptorSetLayout::add_binding(
                        0,
                        vk::DescriptorType::UNIFORM_BUFFER,
                        vk::ShaderStageFlags::VERTEX,
                        Some(1),
                        None 
                    )
                );
                descriptor_fields.insert(name.to_owned(),set_layout);
            }
            else{
                 let set_layout =  DescriptorSetLayout::build(
                    device,
                    DescriptorSetLayout::add_binding(
                        0,
                        vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                        vk::ShaderStageFlags::VERTEX,
                        Some(1),
                        None 
                    )
                ); 
            }
        }

        for (name,values) in parser.frag_structs.iter(){
            shader_structs.insert("FRAG-".to_string() + name, values.clone());
        }

        for (name,values) in &parser.frag_push_constants{
            if push_returns.contains_key(name) && &values == &push_returns.get(name).unwrap(){
                push_fields.get_mut(name).unwrap().stage_flags = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT; 
            }
            else{
                push_returns.insert(name.to_owned(), values.clone());
                let mut max_value = 0;

                for value in values{
                    max_value += parser.convert_to_size(&value.0);
                }
            
                push_fields.insert(name.to_owned(), vk::PushConstantRange { stage_flags: vk::ShaderStageFlags::FRAGMENT, offset: 0, size: max_value as u32 });
            }
        }

        for (name,values) in parser.frag_descriptors.iter(){
            if descriptor_returns.contains_key(name) && &values == &descriptor_returns.get(name).unwrap(){

                let set_layout = if !values.iter().any(|string| string.0.contains("sampler")) {
                    DescriptorSetLayout::build(
                        device,
                        DescriptorSetLayout::add_binding(
                            0,
                            vk::DescriptorType::UNIFORM_BUFFER,
                            vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                            Some(1),
                            None 
                        )
                    )
                }
                else{
                    DescriptorSetLayout::build(
                        device,
                        DescriptorSetLayout::add_binding(
                            0,
                            vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                            vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                            Some(1),
                            None 
                        )
                    )
                };

                descriptor_fields.insert(name.to_owned(),set_layout);
            }
            else{
                descriptor_returns.insert(name.to_owned(), values.clone());

                if !values.iter().any(|string| string.0.contains("sampler")) {
                    let set_layout = DescriptorSetLayout::build(
                        device,
                        DescriptorSetLayout::add_binding(
                            0,
                            vk::DescriptorType::UNIFORM_BUFFER,
                            vk::ShaderStageFlags::VERTEX,
                            Some(1),
                            None 
                        )
                    );
                    descriptor_fields.insert(name.to_owned(),set_layout);
                }
                else{
                    let set_layout =  DescriptorSetLayout::build(
                        device,
                        DescriptorSetLayout::add_binding(
                            0,
                            vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                            vk::ShaderStageFlags::VERTEX,
                            Some(1),
                            None 
                        )
                    ); 
                }
            }
        }
 
        let mut push_values:HashMap<String,Vec<FieldData>> = HashMap::new();

        for (name,values) in push_returns.iter(){
            let mut result_values:Vec<FieldData> = Vec::new();

            for value in values{
                result_values.push(FieldData { name: value.1.clone(), data_type: value.0.clone(), value: Shader::default_value(value.0.clone()) })
            }
            push_values.insert(name.deref().to_string(), result_values);
        }

        let mut descriptor_values:HashMap<String,Vec<FieldData>> = HashMap::new();


        for (name,values) in descriptor_returns.iter(){
            let mut result_values:Vec<FieldData> = Vec::new();
            for value in values{
                result_values.push(FieldData { name: value.1.clone(), data_type: value.0.clone(), value: Shader::default_value(value.0.clone()) })
            }
            descriptor_values.insert(name.deref().to_string(), result_values);
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

    fn default_value(data_type:String) -> Box<dyn Any>{
        match data_type.as_str(){
            "int" => return Box::new(0),
            "uint" => return Box::new(0),
            "float" => return Box::new(0.0),
            "bool" => return Box::new(false),
            "bvec2" => return Box::new(glam::BVec2::default()),
            "bvec3" => return Box::new(glam::BVec3::default()),
            "bvec4" => return Box::new(glam::BVec4::default()),
            "ivec2" => return Box::new(glam::IVec2::default()),
            "ivec3" => return Box::new(glam::IVec3::default()),
            "ivec4" => return Box::new(glam::IVec4::default()),
            "uvec2" => return Box::new(glam::UVec2::default()),
            "uvec3" => return Box::new(glam::UVec3::default()),
            "uvec4" => return Box::new(glam::UVec4::default()),
            "vec2" => return Box::new(glam::Vec2::default()),
            "vec3" => return Box::new(glam::Vec3::default()),
            "vec4" => return Box::new(glam::Vec4::default()),
            "mat2" => return Box::new(glam::Mat2::default()),
            "mat3" => return Box::new(glam::Mat3::default()),
            "mat4" => return Box::new(glam::Mat4::default()),
            _ => return Box::new(()) 
             
        }
    }
}
