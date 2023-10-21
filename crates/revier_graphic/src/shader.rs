use std::{fs::File, io::Read, collections::HashMap, ops::Deref, any::Any};

use ash::vk;

use revier_core::device::Device;
use glsl_parser::parser::Parser;
use revier_data::descriptor::DescriptorSetLayout;
use revier_object::game_object::Component;

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
    pub descriptor_fields:HashMap<String,DescriptorSetLayout>
}

impl Shader {
    pub fn new(device: &Device, vert_file_path: &str, frag_file_path: &str) -> Self {
        let mut push_returns:HashMap<String,Vec<(String,String)>> = HashMap::new();
        let mut descriptor_returns:HashMap<String,Vec<(String,String)>> = HashMap::new();
        let mut push_fields:HashMap<String,vk::PushConstantRange> = HashMap::new();
        let mut descriptor_fields:HashMap<String,DescriptorSetLayout> = HashMap::new();

        let mut parser = Parser::new(); 

        parser.parse_shader(vert_file_path,frag_file_path);

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

        /*println!("{:?}",push_values);
        println!("{:?}",descriptor_values);
        println!("{:?}",push_fields);
        println!("{:?}",descriptor_fields);*/

        return Self {
            vert_module: Shader::create_shader_module(Shader::read_file(String::from(vert_file_path.to_owned() + &".spv".to_owned())), device),
            frag_module: Shader::create_shader_module(Shader::read_file(String::from(frag_file_path.to_owned() + &".spv".to_owned())), device),
            vert_path: String::from(vert_file_path),
            frag_path: String::from(frag_file_path),
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
                        field.value = Box::new(nalgebra::Vector2::new(v1, v2));
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
                    if field.name == parts[1] && field.data_type == "bvec2" {
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
                    if field.name == parts[1] && field.data_type == "bvec3" {
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
                    if field.name == parts[1] && field.data_type == "bvec4" {
                        field.value = Box::new(glam::UVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }
}

impl Component for Shader {}

