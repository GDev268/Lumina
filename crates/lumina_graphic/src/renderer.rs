use std::{collections::HashMap, rc::Rc};

use lumina_core::{
    device::Device,
    swapchain::{self, Swapchain, MAX_FRAMES_IN_FLIGHT},
    window::Window,
};

use lumina_data::descriptor::DescriptorSetLayout;
use lumina_geometry::model::{Model, PushConstantData};
use lumina_object::{game_object::GameObject, transform::Transform};
use lumina_scene::query::Query;

use wgpu::*;

use crate::shader::FieldData;

use super::{
    pipeline::{Pipeline, PipelineConfiguration},
    shader::Shader,
    types::{LuminaShaderType, LuminaShaderTypeConverter},
};

use std::fmt::Debug;

pub struct Renderer{
    pipeline: Option<Pipeline>,
    pipeline_layout: Option<PipelineLayout>
}

impl Renderer {
    pub fn new() -> Self{
        Self { pipeline: None, pipeline_layout: None }
    }

    pub fn begin_frame(device:&Device) {
        let output = device.get_surface().get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = device.device().create_command_encoder(&CommandEncoderDescriptor { label: Some("Render Encoder") });
    }
}