use std::{borrow::BorrowMut, collections::HashMap, fs::File, io::Read, rc::Rc};

use glsl_parser::parser::Parser;
use lumina_core::device::Device;
use lumina_data::type_padding::{
    FloatPadding, IntPadding, Mat2Padding, Mat3Padding, Mat4Padding, Vec2Padding, Vec3Padding,
    Vec4Padding,
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    ShaderModuleDescriptor,
};

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

#[derive(Debug)]
pub struct ConstantComponents<'a> {
    pub buffer: wgpu::Buffer,
    pub layout: wgpu::VertexBufferLayout<'a>,
}

pub struct Shader<'a> {
    pub shader_module: wgpu::ShaderModule,
    pub vert_path: String,
    pub frag_path: String,
    pub constant_values: HashMap<String, Vec<FieldData>>,
    pub descriptor_values: HashMap<String, Vec<FieldData>>,
    pub descriptor_images: HashMap<String, wgpu::Texture>,
    pub constant_fields: HashMap<String, ConstantComponents<'a>>,
    pub descriptor_fields: HashMap<String, DescriptorComponents>,
    pub value_sizes: HashMap<String, (usize, u16)>,
}

impl<'a> Shader<'a> {
    pub fn new(device: &Device, shader_file_path: &str) /*-> Self*/
    {
        let mut shader_code = Shader::read_file(shader_file_path);

        let shader_lines: Vec<&str> = shader_code.lines().collect();

        let mut result: Vec<&str> = shader_lines.into_iter().collect();

        let mut constant_returns: HashMap<String, Vec<(String, String)>> = HashMap::new();
        let mut constant_fields: HashMap<String, ConstantComponents<'a>> = HashMap::new();

        let cur_binding: u32 = 5;

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

        let mut attributes: Vec<wgpu::VertexAttribute>;
        for (name, values) in parser.wgsl_constants_vert {
            constant_returns.insert(name.to_owned(), values.clone());

            if !values
                .iter()
                .any(|string| string.0.contains("texture_2d") || string.0.contains("sampler"))
            {
                let mut default_values: Vec<LuminaShaderType> = Vec::new();
                let mut buffer_size: wgpu::BufferAddress = 0;

                for value in values {
                    default_values.push(Shader::default_value(value.0));
                }

                for value in default_values.iter() {
                    buffer_size += match value {
                        LuminaShaderType::INT(value) => {
                            std::mem::size_of::<IntPadding>() as wgpu::BufferAddress
                        }
                        LuminaShaderType::UINT(value) => todo!(),
                        LuminaShaderType::FLOAT(value) => {
                            std::mem::size_of::<FloatPadding>() as wgpu::BufferAddress
                        }
                        LuminaShaderType::BOOL(_) => todo!(),
                        LuminaShaderType::BVEC2(_) => todo!(),
                        LuminaShaderType::BVEC3(_) => todo!(),
                        LuminaShaderType::BVEC4(_) => todo!(),
                        LuminaShaderType::IVEC2(_) => todo!(),
                        LuminaShaderType::IVEC3(_) => todo!(),
                        LuminaShaderType::IVEC4(_) => todo!(),
                        LuminaShaderType::UVEC2(_) => todo!(),
                        LuminaShaderType::UVEC3(_) => todo!(),
                        LuminaShaderType::UVEC4(_) => todo!(),
                        LuminaShaderType::VEC2(value) => {
                            std::mem::size_of::<Vec2Padding>() as wgpu::BufferAddress
                        }
                        LuminaShaderType::VEC3(value) => {
                            std::mem::size_of::<Vec3Padding>() as wgpu::BufferAddress
                        }
                        LuminaShaderType::VEC4(value) => {
                            std::mem::size_of::<Vec4Padding>() as wgpu::BufferAddress
                        }
                        LuminaShaderType::MAT2(value) => {
                            std::mem::size_of::<Mat2Padding>() as wgpu::BufferAddress
                        }
                        LuminaShaderType::MAT3(value) => {
                            std::mem::size_of::<Mat3Padding>() as wgpu::BufferAddress
                        }
                        LuminaShaderType::MAT4(value) => {
                            std::mem::size_of::<Mat4Padding>() as wgpu::BufferAddress
                        }
                    }
                }

                let attributes = Shader::create_vertex_attributes(
                    cur_binding,
                    &default_values,
                );

                // Clone the Rc when creating layout
                let layout = wgpu::VertexBufferLayout {
                    array_stride: buffer_size,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &attributes,
                };

                let buffer = device.device().create_buffer_init(&BufferInitDescriptor {
                    label: Some((name.clone() + "_Buffer").as_str()),
                    contents: &LuminaShaderType::to_ne_bytes(default_values),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
                
                constant_fields.insert(
                    name.clone().to_owned(),
                    ConstantComponents { buffer, layout },
                );
            }
        }

        /*return Self {
            shader_module: device.device().create_shader_module(ShaderModuleDescriptor{label: Some("Vertex Module"), source: wgpu::ShaderSource::Wgsl(Shader::read_file(shader_file_path).into())}),
        };*/
    }


    fn create_vertex_attributes(
        mut cur_binding: u32,
        values: &Vec<LuminaShaderType>,
    ) -> Vec<wgpu::VertexAttribute> {
        let mut attributes: Vec<wgpu::VertexAttribute> = Vec::new();
        let mut offset: wgpu::BufferAddress = 0;
        for value in values {
            match value {
                LuminaShaderType::INT(value) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Sint32,
                        offset,
                        shader_location: cur_binding,
                    });
                    offset += std::mem::size_of::<IntPadding>() as wgpu::BufferAddress;
                    cur_binding += 1;
                }
                LuminaShaderType::UINT(_) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Uint32,
                        offset,
                        shader_location: cur_binding,
                    });
                    offset += std::mem::size_of::<u32>() as wgpu::BufferAddress;
                    cur_binding += 1;
                }
                LuminaShaderType::FLOAT(_) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32,
                        offset,
                        shader_location: cur_binding,
                    });
                    offset += std::mem::size_of::<FloatPadding>() as wgpu::BufferAddress;
                    cur_binding += 1;
                }
                LuminaShaderType::BOOL(_) => todo!(),
                LuminaShaderType::BVEC2(_) => todo!(),
                LuminaShaderType::BVEC3(_) => todo!(),
                LuminaShaderType::BVEC4(_) => todo!(),
                LuminaShaderType::IVEC2(_) => todo!(),
                LuminaShaderType::IVEC3(_) => todo!(),
                LuminaShaderType::IVEC4(_) => todo!(),
                LuminaShaderType::UVEC2(_) => todo!(),
                LuminaShaderType::UVEC3(_) => todo!(),
                LuminaShaderType::UVEC4(_) => todo!(),
                LuminaShaderType::VEC2(_) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset,
                        shader_location: cur_binding,
                    });
                    offset += std::mem::size_of::<Vec2Padding>() as wgpu::BufferAddress;
                    cur_binding += 1;
                }
                LuminaShaderType::VEC3(_) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset,
                        shader_location: cur_binding,
                    });
                    offset += std::mem::size_of::<Vec3Padding>() as wgpu::BufferAddress;
                    cur_binding += 1;
                }
                LuminaShaderType::VEC4(_) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x4,
                        offset,
                        shader_location: cur_binding,
                    });
                    offset += std::mem::size_of::<Vec4Padding>() as wgpu::BufferAddress;
                    cur_binding += 1;
                }
                LuminaShaderType::MAT2(_) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset,
                        shader_location: cur_binding,
                    });
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: offset + std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        shader_location: cur_binding + 1,
                    });
                    offset += std::mem::size_of::<Mat2Padding>() as wgpu::BufferAddress;
                    cur_binding += 2;
                }
                LuminaShaderType::MAT3(_) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset,
                        shader_location: cur_binding,
                    });
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: offset + std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        shader_location: cur_binding + 1,
                    });
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: offset + std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                        shader_location: cur_binding + 2,
                    });
                    offset += std::mem::size_of::<Mat3Padding>() as wgpu::BufferAddress;
                    cur_binding += 3;
                }
                LuminaShaderType::MAT4(_) => {
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset,
                        shader_location: cur_binding,
                    });
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: offset + std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                        shader_location: cur_binding + 1,
                    });
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: offset + std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                        shader_location: cur_binding + 2,
                    });
                    attributes.push(wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: offset + std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                        shader_location: cur_binding + 3,
                    });
                    offset += std::mem::size_of::<Mat4Padding>() as wgpu::BufferAddress;
                    cur_binding += 4;
                }
            }
        }

        return attributes;
    }

    fn default_value(data_type: String) -> LuminaShaderType {
        match data_type.as_str() {
            "i32" => return LuminaShaderType::INT(IntPadding::default()),
            "u32" => return LuminaShaderType::UINT(0),
            "f32" => return LuminaShaderType::FLOAT(FloatPadding::default()),
            "bool" => return LuminaShaderType::BOOL(false),
            "vec2<bool>" => return LuminaShaderType::BVEC2([false; 2]),
            "vec3<bool>" => return LuminaShaderType::BVEC3([false; 3]),
            "vec4<bool>" => return LuminaShaderType::BVEC4([false; 4]),
            "vec2<i32>" => return LuminaShaderType::IVEC2([0; 2]),
            "vec3<i32>" => return LuminaShaderType::IVEC3([0; 3]),
            "vec4<i32>" => return LuminaShaderType::IVEC4([0; 4]),
            "vec2<u32>" => return LuminaShaderType::UVEC2([0; 2]),
            "vec3<u32>" => return LuminaShaderType::UVEC3([0; 3]),
            "vec4<u32>" => return LuminaShaderType::UVEC4([0; 4]),
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
