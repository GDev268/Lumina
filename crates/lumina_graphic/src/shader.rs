use std::{any::Any, collections::HashMap, fs::File, io::Read, ops::Deref};

use ash::vk::{self, DescriptorSetLayoutCreateFlags};

use glsl_parser::parser::Parser;
use lumina_core::{device::Device, image::Image, swapchain::MAX_FRAMES_IN_FLIGHT};
use lumina_data::{
    buffer::{self, Buffer},
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, PoolConfig, LayoutConfig},
};
use lumina_object::game_object::Component;
use lumina_scene::GlobalUBO;

use crate::types::LuminaShaderType;

#[derive(Debug)]
pub struct FieldData {
    pub name: String,
    pub data_type: String,
    pub value: LuminaShaderType,
}

pub struct DescriptorComponents {
    pub binding: u32,
    pub buffers: Vec<Buffer>,
    pub is_image: bool,
}

pub struct Shader {
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
    pub vert_path: String,
    pub frag_path: String,
    pub push_values: HashMap<String, Vec<FieldData>>,
    pub descriptor_values: HashMap<String, Vec<FieldData>>,
    pub descriptor_images: HashMap<String, Image>,
    pub push_fields: HashMap<String, vk::PushConstantRange>,
    pub descriptor_fields: HashMap<String, DescriptorComponents>,
    pub shader_descriptor_layout: DescriptorSetLayout,
    pub shader_descriptor_sets: Vec<vk::DescriptorSet>,
    pub value_sizes: HashMap<String, (usize, u16)>,
    pool: DescriptorPool,
}

impl Shader {
    pub fn new(
        device: &Device,
        vert_file_path: &str,
        frag_file_path: &str,
        pool_config: PoolConfig,
    ) -> Self {
        let mut push_returns: HashMap<String, Vec<(String, String)>> = HashMap::new();
        let mut descriptor_returns: HashMap<String, Vec<(String, String)>> = HashMap::new();
        let mut push_fields: HashMap<String, vk::PushConstantRange> = HashMap::new();
        let mut descriptor_fields: HashMap<String, DescriptorComponents> = HashMap::new();
        let mut descriptor_images: HashMap<String, Image> = HashMap::new();
        let mut descriptor_layout_config: LayoutConfig = LayoutConfig::new();
        let mut shader_descriptor_writer: DescriptorWriter = DescriptorWriter::new();
        let mut value_sizes: HashMap<String, (usize, u16)> = HashMap::new();
        let mut cur_offset: u16 = 0;

        let pool = pool_config.build(device);

        let mut parser = Parser::new();

        parser.parse_shader(vert_file_path, frag_file_path);

        for (name, values) in parser.glsl_push_constants.iter() {
            push_returns.insert(name.to_owned(), values.clone());
            let mut max_value = 0;

            for value in values {
                max_value += parser.convert_to_size(&value.0);
            }

            push_fields.insert(
                name.to_owned(),
                vk::PushConstantRange {
                    stage_flags: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                    offset: 0,
                    size: max_value as u32,
                },
            );

            value_sizes.insert("PUSH-".to_string() + name, (max_value, cur_offset));
        }

        for (name, values) in parser.glsl_descriptors.iter() {
            descriptor_returns.insert(name.to_owned(), values.clone());

            if !values.iter().any(|string| string.0.contains("sampler")) {
                let mut components: DescriptorComponents = DescriptorComponents {
                    buffers: Vec::new(),
                    binding: parser.descriptor_data.get(name).unwrap().1.unwrap(),
                    is_image: false
                };

                let mut max_value = 0;

                for value in values {
                    max_value += parser.convert_to_size(&value.0);
                    value_sizes.insert("DESCRIPTOR-".to_string() + name, (max_value, 0));
                }

                for i in 0..lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT {
                    let mut buffer = Buffer::new(
                        &device,
                        max_value as u64,
                        1,
                        ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
                        ash::vk::MemoryPropertyFlags::HOST_VISIBLE,
                    );
                    buffer.map(&device, None, None);

                    components.buffers.push(buffer);
                }

                descriptor_layout_config.add_binding(
                    parser.descriptor_data.get(name).unwrap().1.unwrap(),
                    vk::DescriptorType::UNIFORM_BUFFER,
                    vk::ShaderStageFlags::ALL_GRAPHICS,
                    1,
                );

                descriptor_fields.insert(name.to_owned(), components);
            } else {
                let mut max_value = 0;

                let mut components: DescriptorComponents = DescriptorComponents {
                    buffers: Vec::new(),
                    binding: 0,
                    is_image: true
                };

                descriptor_layout_config.add_binding(
                    parser.descriptor_data.get(name).unwrap().1.unwrap(),
                    vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    vk::ShaderStageFlags::ALL_GRAPHICS,
                    1,
                );

                descriptor_images.insert(name.to_owned(), Image::default());

                descriptor_fields.insert(name.to_owned(), components);
            }
        }

        let mut push_values: HashMap<String, Vec<FieldData>> = HashMap::new();

        for (name, values) in push_returns.iter() {
            let mut result_values: Vec<FieldData> = Vec::new();

            for value in values {
                result_values.push(FieldData {
                    name: value.1.clone(),
                    data_type: value.0.clone(),
                    value: Shader::default_value(value.0.clone()),
                })
            }
            push_values.insert(name.deref().to_string(), result_values);
        }

        let mut descriptor_values: HashMap<String, Vec<FieldData>> = HashMap::new();

        for (name, values) in descriptor_returns.iter() {
            let mut result_values: Vec<FieldData> = Vec::new();
            for value in values {
                result_values.push(FieldData {
                    name: value.1.clone(),
                    data_type: value.0.clone(),
                    value: Shader::default_value(value.0.clone()),
                })
            }
            descriptor_values.insert(name.deref().to_string(), result_values);
        }

        let shader_descriptor_layout = descriptor_layout_config.build(device);

        let mut shader_descriptor_sets: Vec<vk::DescriptorSet> = Vec::new();
        for i in 0..lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT {
            let mut descriptor_writer = DescriptorWriter::new();

            for (name, components) in descriptor_fields.iter() {
                if components.is_image {
                    descriptor_writer.write_image(
                        components.binding,
                        descriptor_images.get(name).unwrap().descriptor_info(),
                        &shader_descriptor_layout,
                    );
                } else {
                    descriptor_writer.write_buffer(
                        components.binding,
                        components.buffers[i].descriptor_info(None, None),
                        &shader_descriptor_layout,
                    )
                }
            }

            shader_descriptor_sets.push(descriptor_writer.build(
                device,
                shader_descriptor_layout.get_descriptor_set_layout(),
                &pool,
            ));
        }

        return Self {
            vert_module: Shader::create_shader_module(
                Shader::read_file(String::from(vert_file_path.to_owned() + &".spv".to_owned())),
                device,
            ),
            frag_module: Shader::create_shader_module(
                Shader::read_file(String::from(frag_file_path.to_owned() + &".spv".to_owned())),
                device,
            ),
            vert_path: String::from(vert_file_path),
            frag_path: String::from(frag_file_path),
            push_values,
            descriptor_values,
            push_fields,
            descriptor_fields,
            value_sizes,
            pool,
            descriptor_images,
            shader_descriptor_layout,
            shader_descriptor_sets,
        };
    }

    pub fn create_descriptor_sets(&mut self) {}

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

    fn default_value(data_type: String) -> LuminaShaderType {
        match data_type.as_str() {
            "int" => return LuminaShaderType::INT(0),
            "uint" => return LuminaShaderType::UINT(0),
            "float" => return LuminaShaderType::FLOAT(0.0),
            "bool" => return LuminaShaderType::BOOL(false),
            "bvec2" => return LuminaShaderType::BVEC2(glam::BVec2::FALSE),
            "bvec3" => return LuminaShaderType::BVEC3(glam::BVec3::FALSE),
            "bvec4" => return LuminaShaderType::BVEC4(glam::BVec4::FALSE),
            "ivec2" => return LuminaShaderType::IVEC2(glam::IVec2::ZERO),
            "ivec3" => return LuminaShaderType::IVEC3(glam::IVec3::ZERO),
            "ivec4" => return LuminaShaderType::IVEC4(glam::IVec4::ZERO),
            "uvec2" => return LuminaShaderType::UVEC2(glam::UVec2::ZERO),
            "uvec3" => return LuminaShaderType::UVEC3(glam::UVec3::ZERO),
            "uvec4" => return LuminaShaderType::UVEC4(glam::UVec4::ZERO),
            "vec2" => return LuminaShaderType::VEC2(glam::Vec2::ZERO),
            "vec3" => return LuminaShaderType::VEC3(glam::Vec3::ZERO),
            "vec4" => return LuminaShaderType::VEC4(glam::Vec4::ZERO),
            "mat2" => return LuminaShaderType::MAT2(glam::Mat2::ZERO),
            "mat3" => return LuminaShaderType::MAT3(glam::Mat3::ZERO),
            "mat4" => return LuminaShaderType::MAT4(glam::Mat4::ZERO),
            _ => panic!("ERROR: Failed to set an default value!"),
        }
    }

    pub fn change_uniform_1f(&mut self, location: &str, v1: f32) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "float" {
                        field.value = LuminaShaderType::FLOAT(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "float" {
                        field.value = LuminaShaderType::FLOAT(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_2f(&mut self, location: &str, v1: f32, v2: f32) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec2" {
                        field.value = LuminaShaderType::VEC2(glam::Vec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec2" {
                        field.value = LuminaShaderType::VEC2(glam::Vec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_3f(
        &mut self,
        location: &str,
        v1: f32,
        v2: f32,
        v3: f32,
    ) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec3" {
                        field.value = LuminaShaderType::VEC3(glam::Vec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_4f(
        &mut self,
        location: &str,
        v1: f32,
        v2: f32,
        v3: f32,
        v4: f32,
    ) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec4" {
                        field.value = LuminaShaderType::VEC4(glam::Vec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec4" {
                        field.value = LuminaShaderType::VEC4(glam::Vec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_vec2(&mut self, location: &str, v1: glam::Vec2) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec2" {
                        field.value = LuminaShaderType::VEC2(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec2" {
                        field.value = LuminaShaderType::VEC2(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_vec3(&mut self, location: &str, v1: glam::Vec3) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec3" {
                        field.value = LuminaShaderType::VEC3(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec3" {
                        field.value = LuminaShaderType::VEC3(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_vec4(&mut self, location: &str, v1: glam::Vec4) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec4" {
                        field.value = LuminaShaderType::VEC4(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "vec4" {
                        field.value = LuminaShaderType::VEC4(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_1i(&mut self, location: &str, v1: i32) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "int" {
                        field.value = LuminaShaderType::INT(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "int" {
                        field.value = LuminaShaderType::INT(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_2i(&mut self, location: &str, v1: i32, v2: i32) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec2" {
                        field.value = LuminaShaderType::IVEC2(glam::IVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec2" {
                        field.value = LuminaShaderType::IVEC2(glam::IVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_3i(
        &mut self,
        location: &str,
        v1: i32,
        v2: i32,
        v3: i32,
    ) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec3" {
                        field.value = LuminaShaderType::IVEC3(glam::IVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec3" {
                        field.value = LuminaShaderType::IVEC3(glam::IVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_4i(
        &mut self,
        location: &str,
        v1: i32,
        v2: i32,
        v3: i32,
        v4: i32,
    ) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec4" {
                        field.value = LuminaShaderType::IVEC4(glam::IVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec4" {
                        field.value = LuminaShaderType::IVEC4(glam::IVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_ivec2(&mut self, location: &str, v1: glam::IVec2) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec2" {
                        field.value = LuminaShaderType::IVEC2(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec2" {
                        field.value = LuminaShaderType::IVEC2(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_ivec3(&mut self, location: &str, v1: glam::IVec3) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec3" {
                        field.value = LuminaShaderType::IVEC3(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec3" {
                        field.value = LuminaShaderType::IVEC3(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_ivec4(&mut self, location: &str, v1: glam::IVec4) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec4" {
                        field.value = LuminaShaderType::IVEC4(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "ivec4" {
                        field.value = LuminaShaderType::IVEC4(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_1b(&mut self, location: &str, v1: bool) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bool" {
                        field.value = LuminaShaderType::BOOL(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bool" {
                        field.value = LuminaShaderType::BOOL(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_2b(&mut self, location: &str, v1: bool, v2: bool) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec2" {
                        field.value = LuminaShaderType::BVEC2(glam::BVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec2" {
                        field.value = LuminaShaderType::BVEC2(glam::BVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_3b(
        &mut self,
        location: &str,
        v1: bool,
        v2: bool,
        v3: bool,
    ) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec3" {
                        field.value = LuminaShaderType::BVEC3(glam::BVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec3" {
                        field.value = LuminaShaderType::BVEC3(glam::BVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_4b(
        &mut self,
        location: &str,
        v1: bool,
        v2: bool,
        v3: bool,
        v4: bool,
    ) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec4" {
                        field.value = LuminaShaderType::BVEC4(glam::BVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec4" {
                        field.value = LuminaShaderType::BVEC4(glam::BVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_bvec2(&mut self, location: &str, v1: glam::BVec2) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec2" {
                        field.value = LuminaShaderType::BVEC2(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec2" {
                        field.value = LuminaShaderType::BVEC2(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_bvec3(&mut self, location: &str, v1: glam::BVec3) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec3" {
                        field.value = LuminaShaderType::BVEC3(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec3" {
                        field.value = LuminaShaderType::BVEC3(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_bvec4(&mut self, location: &str, v1: glam::BVec4) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec4" {
                        field.value = LuminaShaderType::BVEC4(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "bvec4" {
                        field.value = LuminaShaderType::BVEC4(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_1u(&mut self, location: &str, v1: u32) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uint" {
                        field.value = LuminaShaderType::UINT(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uint" {
                        field.value = LuminaShaderType::UINT(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_2u(&mut self, location: &str, v1: u32, v2: u32) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec2" {
                        field.value = LuminaShaderType::UVEC2(glam::UVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec2" {
                        field.value = LuminaShaderType::UVEC2(glam::UVec2::new(v1, v2));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_3u(
        &mut self,
        location: &str,
        v1: u32,
        v2: u32,
        v3: u32,
    ) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec3" {
                        field.value = LuminaShaderType::UVEC3(glam::UVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec3" {
                        field.value = LuminaShaderType::UVEC3(glam::UVec3::new(v1, v2, v3));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_4u(
        &mut self,
        location: &str,
        v1: u32,
        v2: u32,
        v3: u32,
        v4: u32,
    ) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec4" {
                        field.value = LuminaShaderType::UVEC4(glam::UVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec4" {
                        field.value = LuminaShaderType::UVEC4(glam::UVec4::new(v1, v2, v3, v4));
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_uvec2(&mut self, location: &str, v1: glam::UVec2) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec2" {
                        field.value = LuminaShaderType::UVEC2(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec2" {
                        field.value = LuminaShaderType::UVEC2(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_uvec3(&mut self, location: &str, v1: glam::UVec3) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec3" {
                        field.value = LuminaShaderType::UVEC3(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec3" {
                        field.value = LuminaShaderType::UVEC3(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_uvec4(&mut self, location: &str, v1: glam::UVec4) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec4" {
                        field.value = LuminaShaderType::UVEC4(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "uvec4" {
                        field.value = LuminaShaderType::UVEC4(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_mat2(&mut self, location: &str, v1: glam::Mat2) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat2" {
                        field.value = LuminaShaderType::MAT2(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat2" {
                        field.value = LuminaShaderType::MAT2(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_mat3(&mut self, location: &str, v1: glam::Mat3) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat3" {
                        field.value = LuminaShaderType::MAT3(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat3" {
                        field.value = LuminaShaderType::MAT3(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }

    pub fn change_uniform_mat4(&mut self, location: &str, v1: glam::Mat4) -> Result<(), &str> {
        let parts: Vec<&str> = location.splitn(2, ".").collect();

        for (name, fields) in self.push_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat4" {
                        field.value = LuminaShaderType::MAT4(v1);
                        return Ok(());
                    }
                }
            }
        }

        for (name, fields) in self.descriptor_values.iter_mut() {
            if name == parts[0] {
                for field in fields {
                    if field.name == parts[1] && field.data_type == "mat4" {
                        field.value = LuminaShaderType::MAT4(v1);
                        return Ok(());
                    }
                }
            }
        }

        return Err("Failed to get the value!");
    }
}

impl Component for Shader {}
