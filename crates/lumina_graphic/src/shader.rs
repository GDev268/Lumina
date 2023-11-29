use std::{fs::File, io::Read, collections::HashMap};

use lumina_core::device::Device;
use wgpu::ShaderModuleDescriptor;

use crate::types::LuminaShaderType;

#[derive(Debug)]
pub struct FieldData {
    pub name: String,
    pub data_type: String,
    pub value: LuminaShaderType,
}

pub struct DescriptorComponents {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffers:  wgpu::Buffer,
    pub bind_group: wgpu::BindGroup
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
    pub fn new(/*device: &Device,*/ shader_file_path: &str) /*-> Self*/ {
        let mut shader_code = Shader::read_file(shader_file_path);

        let shader_lines:Vec<&str> = shader_code.lines().collect();

        let mut result:Vec<&str> = shader_lines.into_iter().collect();

        let mut cur_stage = "";
        for mut line in result.iter_mut() {

            match *line{
                "#Vertex" => {
                    cur_stage = "vert";
                    *line = "";
                },
                "#Fragment" => {
                    cur_stage = "frag";
                    *line = "";
                },
                "#Compute" => {
                    cur_stage = "comp";
                    *line = "";
                },
                "@Constant" => {
                    *line = "";
                }
                "@Uniform" => {
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
            }
            else if *line == "#end" && cur_stage == "frag" {
                *line = "@fragment";
            }
        }

        shader_code = result.join("\n");

        //println!("{:?}",shader_code);
        //panic!("");

        /*return Self {
            shader_module: device.device().create_shader_module(ShaderModuleDescriptor{label: Some("Vertex Module"), source: wgpu::ShaderSource::Wgsl(Shader::read_file(shader_file_path).into())}),
        };*/
    }

    pub fn read_file(file_path: &str) -> String {
        let mut file = File::open(file_path).expect("Failed to open shader file");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to convert shader to string");

        return contents;
    }
}
