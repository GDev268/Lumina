use std::{fs::File, io::Read, collections::HashMap, ops::Deref, any::Any};

use ash::vk;

use revier_core::device::Device;
use glsl_parser::parser::Parser;
use revier_data::descriptor::DescriptorSetLayout;
use revier_object::game_object::Component;

pub enum ConvertType{
    INT(i32),
    UINT(u32),
    FLOAT(f32),
    BOOL(bool),
    BVEC2(glam::BVec2),
    BVEC3(glam::BVec3),
    BVEC4(glam::BVec4),
    IVEC2(glam::IVec2),
    IVEC3(glam::IVec3),
    IVEC4(glam::IVec4),
    UVEC2(glam::UVec2),
    UVEC3(glam::UVec3),
    UVEC4(glam::UVec4),
    VEC2(glam::Vec2),
    VEC3(glam::Vec3),
    VEC4(glam::Vec4),
    MAT2(glam::Mat2),
    MAT3(glam::Mat3),
    MAT4(glam::Mat4),
}

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
    pub push_values:HashMap<String,Vec<FieldData>>,
    pub descriptor_values:HashMap<String,Vec<FieldData>>,
    pub push_fields:HashMap<String,vk::PushConstantRange>,
    pub descriptor_fields:HashMap<String,DescriptorSetLayout>,
    pub value_sizes:HashMap<String,(u8,u16)>,   
}

impl Shader {
    pub fn new(device: &Device, vert_file_path: &str, frag_file_path: &str) -> Self {
        let mut push_returns:HashMap<String,Vec<(String,String)>> = HashMap::new();
        let mut descriptor_returns:HashMap<String,Vec<(String,String)>> = HashMap::new();
        let mut push_fields:HashMap<String,vk::PushConstantRange> = HashMap::new();
        let mut descriptor_fields:HashMap<String,DescriptorSetLayout> = HashMap::new();
        let mut value_sizes:HashMap<String,(u8,u16)> = HashMap::new();
        let mut cur_offset = 0;

        let mut parser = Parser::new(); 

        parser.parse_shader(vert_file_path,frag_file_path);
        println!("{:?}",parser.vert_push_constants);

        for (name,values) in parser.vert_push_constants.iter(){
            push_returns.insert(name.to_owned(), values.clone());
            let mut max_value = 0;

            for value in values{
                max_value += parser.convert_to_size(&value.0);
            }

            value_sizes.insert("PUSH-".to_string() + name, (max_value,cur_offset));
            cur_offset += max_value as u16;

            push_fields.insert(name.to_owned(), vk::PushConstantRange { stage_flags: vk::ShaderStageFlags::VERTEX, offset: 0, size: max_value as u32 });
        }


        for (name,values) in parser.vert_descriptors.iter(){
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

                let mut max_value = 0;

                for value in values{
                    max_value += parser.convert_to_size(&value.0);
                    value_sizes.insert("DESCRIPTOR-".to_string() + name, (max_value,0));
                }                
            }
            else{
                let mut max_value = 0;

                for value in values{
                    max_value += parser.convert_to_size(&value.0);
                    value_sizes.insert("DESCRIPTOR-".to_string() + name, (max_value,0));
                }

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

        for (name,values) in &parser.frag_push_constants{
            if push_returns.contains_key(name) && &values == &push_returns.get(name).unwrap(){
                push_fields.get_mut(name).unwrap().stage_flags = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;
            }
            else{
                push_returns.insert(name.to_owned(), values.clone());
                let mut max_value = 0;

                for value in values{
                    max_value += parser.convert_to_size(&value.0);
                    value_sizes.insert("PUSH-".to_string() + name, (max_value,cur_offset));
                }
            
                push_fields.insert(name.to_owned(), vk::PushConstantRange { stage_flags: vk::ShaderStageFlags::FRAGMENT, offset: 0, size: max_value as u32 });
            }
        }

        for (name,values) in parser.frag_descriptors.iter(){
            if descriptor_returns.contains_key(name) && &values == &descriptor_returns.get(name).unwrap(){
                let set_layout = if !values.iter().any(|string| string.0.contains("sampler")) {
                    let mut max_value = 0;

                    for value in values{
                        max_value += parser.convert_to_size(&value.0);
                        value_sizes.insert("DESCRIPTOR-".to_string() + name, (max_value,0));
                    }

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
                    let mut max_value = 0;

                    for value in values{
                        max_value += parser.convert_to_size(&value.0);
                        value_sizes.insert("DESCRIPTOR-".to_string() + name, (max_value,0));
                    }

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
                let mut max_value = 0;

                for value in values{
                    max_value += parser.convert_to_size(&value.0);
                    value_sizes.insert("DESCRIPTOR-".to_string() + name, (max_value,0));
                }

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
                    let mut max_value = 0;

                    for value in values{
                        max_value += parser.convert_to_size(&value.0);
                        value_sizes.insert("DESCRIPTOR-".to_string() + name, (max_value,0));
                    }

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

        /*println!("{:?}",push_values);
        println!("{:?}",descriptor_values);
        println!("{:?}",push_fields);
        println!("{:?}",descriptor_fields);*/
        println!("{:?}",value_sizes);

        return Self {
            vert_module: Shader::create_shader_module(Shader::read_file(String::from(vert_file_path.to_owned() + &".spv".to_owned())), device),
            frag_module: Shader::create_shader_module(Shader::read_file(String::from(frag_file_path.to_owned() + &".spv".to_owned())), device),
            vert_path: String::from(vert_file_path),
            frag_path: String::from(frag_file_path),
            push_values,
            descriptor_values,
            push_fields,
            descriptor_fields,
            value_sizes
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
            "int" => return Box::new(()),
            "uint" => return Box::new(()),
            "float" => return Box::new(()),
            "bool" => return Box::new(()),
            "bvec2" => return Box::new(()),
            "bvec3" => return Box::new(()),
            "bvec4" => return Box::new(()),
            "ivec2" => return Box::new(()),
            "ivec3" => return Box::new(()),
            "ivec4" => return Box::new(()),
            "uvec2" => return Box::new(()),
            "uvec3" => return Box::new(()),
            "uvec4" => return Box::new(()),
            "vec2" => return Box::new(()),
            "vec3" => return Box::new(()),
            "vec4" => return Box::new(()),
            "mat2" => return Box::new(()),
            "mat3" => return Box::new(()),
            "mat4" => return Box::new(()),
            _ => return Box::new(()) 
             
        }
    }

    pub fn get_field_value<T:'static>(field_data:&FieldData) -> Option<ConvertType>{
        match field_data.data_type.as_str(){
            "int" => 
                if let Some(value) = field_data.value.downcast_ref::<i32>(){
                    return Some(ConvertType::INT(*value));
                }
                else{
                    return Some(ConvertType::INT(0));
                },
            "uint" => 
                if let Some(value) = field_data.value.downcast_ref::<u32>(){
                    return Some(ConvertType::UINT(*value));
                }
                else{
                    return Some(ConvertType::UINT(0));
                },
            "float" => 
                if let Some(value) = field_data.value.downcast_ref::<f32>(){
                    return Some(ConvertType::FLOAT(*value));
                }
                else{

                    return Some(ConvertType::FLOAT(0.0));
                },
            "bool" => 
                if let Some(value) = field_data.value.downcast_ref::<bool>(){
                    return Some(ConvertType::BOOL(*value));
                }
                else{
                    return Some(ConvertType::BOOL(false));
                },
            "bvec2" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::BVec2>(){
                    return Some(ConvertType::BVEC2(*value));
                }
                else{
                    return Some(ConvertType::BVEC2(glam::BVec2::default()));
                },
            "bvec3" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::BVec3>(){
                    return Some(ConvertType::BVEC3(*value));
                }
                else{
                    return Some(ConvertType::BVEC3(glam::BVec3::default()));
                },
            "bvec4" =>
                if let Some(value) = field_data.value.downcast_ref::<glam::BVec4>(){
                    return Some(ConvertType::BVEC4(*value));
                }
                else{
                    return Some(ConvertType::BVEC4(glam::BVec4::default()));
                },        
            "ivec2" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::IVec2>(){
                    return Some(ConvertType::IVEC2(*value));
                }
                else{
                    return Some(ConvertType::IVEC2(glam::IVec2::default()));
                },
            "ivec3" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::IVec3>(){
                    return Some(ConvertType::IVEC3(*value));
                }
                else{
                    return Some(ConvertType::IVEC3(glam::IVec3::default()));
                },
            "ivec4" =>
                if let Some(value) = field_data.value.downcast_ref::<glam::IVec4>(){
                    return Some(ConvertType::IVEC4(*value));
                }
                else{
                    return Some(ConvertType::IVEC4(glam::IVec4::default()));
                },
            "uvec2" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::UVec2>(){
                    return Some(ConvertType::UVEC2(*value));
                }
                else{
                    return Some(ConvertType::UVEC2(glam::UVec2::default()));
                },
            "uvec3" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::UVec3>(){
                    return Some(ConvertType::UVEC3(*value));
                }
                else{
                    return Some(ConvertType::UVEC3(glam::UVec3::default()));
                },
            "uvec4" =>
                if let Some(value) = field_data.value.downcast_ref::<glam::UVec4>(){
                    return Some(ConvertType::UVEC4(*value));
                }
                else{
                    return Some(ConvertType::UVEC4(glam::UVec4::default()));
                },    
            "vec2" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::Vec2>(){
                    return Some(ConvertType::VEC2(*value));
                }
                else{
                    return Some(ConvertType::VEC2(glam::Vec2::default()));
                },
            "vec3" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::Vec3>(){
                    return Some(ConvertType::VEC3(*value));
                }
                else{
                    return Some(ConvertType::VEC3(glam::Vec3::default()));
                },
            "vec4" =>
                if let Some(value) = field_data.value.downcast_ref::<glam::Vec4>(){
                    return Some(ConvertType::VEC4(*value));
                }
                else{
                    return Some(ConvertType::VEC4(glam::Vec4::default()));
                }, 
            "mat2" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::Mat2>(){
                    return Some(ConvertType::MAT2(*value));
                }
                else{
                    return Some(ConvertType::MAT2(glam::Mat2::default()));
                }, 
            "mat3" =>
                if let Some(value) = field_data.value.downcast_ref::<glam::Mat3>(){
                    return Some(ConvertType::MAT3(*value));
                }
                else{
                    return Some(ConvertType::MAT3(glam::Mat3::default()));
                }, 
            "mat4" => 
                if let Some(value) = field_data.value.downcast_ref::<glam::Mat4>(){
                    return Some(ConvertType::MAT4(*value));
                }
                else{
                    return Some(ConvertType::MAT4(glam::Mat4::default()));
                }, 
            _ => None,
        }
    }
    
    pub fn change_uniform_1f(&mut self,location:&str,v1:f32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "float" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_2f(&mut self,location:&str,v1:f32,v2:f32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec2" {
                        field.value = Box::new(glam::Vec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_3f(&mut self,location:&str,v1:f32,v2:f32,v3:f32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec3" {
                        field.value = Box::new(glam::Vec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_4f(&mut self,location:&str,v1:f32,v2:f32,v3:f32,v4:f32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec4" {
                        field.value = Box::new(glam::Vec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_vec2(&mut self,location:&str,v1:glam::Vec2) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec2" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_vec3(&mut self,location:&str,v1:glam::Vec3) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec3" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_vec4(&mut self,location:&str,v1:glam::Vec4) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec4" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_1i(&mut self,location:&str,v1:i32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "int" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_2i(&mut self,location:&str,v1:i32,v2:i32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec2" {
                        field.value = Box::new(glam::IVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_3i(&mut self,location:&str,v1:i32,v2:i32,v3:i32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec3" {
                        field.value = Box::new(glam::IVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_4i(&mut self,location:&str,v1:i32,v2:i32,v3:i32,v4:i32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec4" {
                        field.value = Box::new(glam::IVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_ivec2(&mut self,location:&str,v1:glam::IVec2) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec2" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_ivec3(&mut self,location:&str,v1:glam::IVec3) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec3" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_ivec4(&mut self,location:&str,v1:glam::IVec4) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec4" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_1b(&mut self,location:&str,v1:bool) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bool" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_2b(&mut self,location:&str,v1:bool,v2:bool) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec2" {
                        field.value = Box::new(glam::BVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_3b(&mut self,location:&str,v1:bool,v2:bool,v3:bool) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec3" {
                        field.value = Box::new(glam::BVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_4b(&mut self,location:&str,v1:bool,v2:bool,v3:bool,v4:bool) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec4" {
                        field.value = Box::new(glam::BVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_bvec2(&mut self,location:&str,v1:glam::BVec2) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec2" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_bvec3(&mut self,location:&str,v1:glam::BVec3) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec3" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_bvec4(&mut self,location:&str,v1:glam::BVec4) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec4" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }


    pub fn change_uniform_1u(&mut self,location:&str,v1:u32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uint" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_2u(&mut self,location:&str,v1:u32,v2:u32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec2" {
                        field.value = Box::new(glam::UVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_3u(&mut self,location:&str,v1:u32,v2:u32,v3:u32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec3" {
                        field.value = Box::new(glam::UVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_4u(&mut self,location:&str,v1:u32,v2:u32,v3:u32,v4:u32) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec4" {
                        field.value = Box::new(glam::UVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_uvec2(&mut self,location:&str,v1:glam::UVec2) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec2" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_uvec3(&mut self,location:&str,v1:glam::UVec3) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec3" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_uvec4(&mut self,location:&str,v1:glam::UVec4) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec4" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_mat2(&mut self,location:&str,v1:glam::Mat2) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat2" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_mat3(&mut self,location:&str,v1:glam::Mat3) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat3" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_mat4(&mut self,location:&str,v1:glam::Mat4) -> Result<(),&str> {
        let parts:Vec<&str> = location.splitn(2, ".").collect();
    
        for (name,fields) in self.push_values.iter_mut(){
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat4" {
                        field.value = Box::new(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

}

impl Component for Shader {}

