use std::{collections::HashMap, fs::File, io::Read};

use glsl_parser::parser::Parser;
use lumina_core::device::Device;
use lumina_data::type_padding::{Mat4Padding, Mat2Padding, Vec4Padding, Vec3Padding, Vec2Padding, Mat3Padding, FloatPadding, IntPadding};
use wgpu::{ShaderModuleDescriptor, util::{DeviceExt, BufferInitDescriptor}};

use crate::types::LuminaShaderType;

#[derive(Debug)]
pub struct FieldData {
    pub name: String,
    pub data_type: String,
    pub value: LuminaShaderType,
}

pub struct DescriptorComponents {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

pub struct Shader {
    pub shader_module: wgpu::ShaderModule,
    pub vert_path: String,
    pub frag_path: String,
    pub push_values: HashMap<String, Vec<FieldData>>,
    pub descriptor_values: HashMap<String, Vec<FieldData>>,
    pub descriptor_images: HashMap<String, wgpu::Texture>,
    pub const_fields: HashMap<String, wgpu::Buffer>,
    pub descriptor_fields: HashMap<String, DescriptorComponents>,
    pub value_sizes: HashMap<String, (usize, u16)>,
}

impl Shader {
    pub fn new(device: &Device, shader_file_path: &str) /*-> Self*/
    {
        let mut shader_code = Shader::read_file(shader_file_path);

        let shader_lines: Vec<&str> = shader_code.lines().collect();

        let mut result: Vec<&str> = shader_lines.into_iter().collect();

        let mut push_returns: HashMap<String, Vec<(String, String)>> = HashMap::new();

        let mut cur_stage = "";
        for line in result.iter_mut() {
            match *line {
                "#Vertex" => {
                    cur_stage = "vert";
                    *line = "";
                }
                "#Fragment" => {
                    cur_stage = "frag";
                    *line = "";
                }
                "#Compute" => {
                    cur_stage = "comp";
                    *line = "";
                }
                _ => {}
            }
            if line.contains("#Vertex") {
                cur_stage = "vert";
                *line = "";
            } else if line.contains("#Fragment") {
                cur_stage = "frag";
                *line = "";
            }

            if *line == "#end" && cur_stage == "vert" {
                *line = "@vertex";
            } else if *line == "#end" && cur_stage == "frag" {
                *line = "@fragment";
            }
        }

        shader_code = result.join("\n");

        let mut parser = Parser::new();

        parser.parse_shader(shader_file_path);

        for (name, values) in parser.wgsl_constants_vert {
            push_returns.insert(name.to_owned(), values.clone());

            if !values
                .iter()
                .any(|string| string.0.contains("texture_2d") || string.0.contains("sampler"))
            {
                let mut default_values:Vec<LuminaShaderType> = Vec::new();
                for value in values {
                    default_values.push(Shader::default_value(value.0));
                }


                let buffer = device.device().create_buffer_init(&BufferInitDescriptor{
                    label: Some((name + "_Buffer").as_str()),
                    contents: &LuminaShaderType::to_ne_bytes(default_values),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                });


                
            };
        }

        //println!("{:?}",shader_code);
        //panic!("");

        /*return Self {
            shader_module: device.device().create_shader_module(ShaderModuleDescriptor{label: Some("Vertex Module"), source: wgpu::ShaderSource::Wgsl(Shader::read_file(shader_file_path).into())}),
        };*/
    }

    fn default_value(data_type: String) -> LuminaShaderType {
        match data_type.as_str() {
            "i32" => return LuminaShaderType::INT(IntPadding::default()),
            "u32" => return LuminaShaderType::UINT(0),
            "f32" => return LuminaShaderType::FLOAT(FloatPadding::default()),
            "bool" => return LuminaShaderType::BOOL(false),
            "vec2<bool>" => return LuminaShaderType::BVEC2([false;2]),
            "vec3<bool>" => return LuminaShaderType::BVEC3([false;3]),
            "vec4<bool>" => return LuminaShaderType::BVEC4([false;4]),
            "vec2<i32>" => return LuminaShaderType::IVEC2([0;2]),
            "vec3<i32>" => return LuminaShaderType::IVEC3([0;3]),
            "vec4<i32>" => return LuminaShaderType::IVEC4([0;4]),
            "vec2<u32>" => return LuminaShaderType::UVEC2([0;2]),
            "vec3<u32>" => return LuminaShaderType::UVEC3([0;3]),
            "vec4<u32>" => return LuminaShaderType::UVEC4([0;4]),
            "vec2<f32>" => return LuminaShaderType::VEC2(Vec2Padding::default()),
            "vec3<f32>" => return LuminaShaderType::VEC3(Vec3Padding::default()),
            "vec4<f32>" => return LuminaShaderType::VEC4(Vec4Padding::default()),
            "mat2x2<f32>" => return LuminaShaderType::MAT2(Mat2Padding::default()),
            "mat3x3<f32>" => return LuminaShaderType::MAT3(Mat3Padding::default()),
            "mat4x4<f32>" => return LuminaShaderType::MAT4(Mat4Padding::default()),
            _ => panic!("ERROR: Failed to set an default value!"),
        }
    }

    pub fn read_file(file_path: &str) -> String {
        let mut file = File::open(file_path).expect("Failed to open shader file");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to convert shader to string");

        return contents;
    }
}
