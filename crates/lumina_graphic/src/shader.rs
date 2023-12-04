use std::{borrow::BorrowMut, collections::HashMap, fs::File, io::Read, rc::Rc};

use glsl_parser::parser::Parser;
use lumina_core::device::Device;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    ShaderModuleDescriptor,
};


pub struct Shader {
    pub shader_module: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(device: &Device, shader_file_path: &str) -> Self {
        return Self {
            shader_module: device
                .device()
                .create_shader_module(ShaderModuleDescriptor {
                    label: Some("Vertex Module"),
                    source: wgpu::ShaderSource::Wgsl(Shader::read_file(shader_file_path).into()),
                }),
        };
    }

    pub fn read_file(file_path: &str) -> String {
        let mut file = File::open(file_path).expect("Failed to open shader file");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to convert shader to string");

        return contents;
    }
}
