use std::{collections::HashMap, rc::Rc};

use lumina_core::{
    device::Device,
    window::Window,
};

//use lumina_geometry::model::{Model, PushConstantData};
use lumina_object::{game_object::GameObject, transform::Transform};
use lumina_scene::query::Query;

use wgpu::*;

/*use crate::shader::FieldData;

use super::{
    pipeline::{Pipeline, PipelineConfiguration},
    shader::Shader,
    types::{LuminaShaderType, LuminaShaderTypeConverter},
};*/

use std::fmt::Debug;

pub struct Renderer<'a> {
    command_encoder: Option<wgpu::CommandEncoder>,
    render_pass: Option<wgpu::RenderPass<'a>>,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Self {
            command_encoder: None,
            render_pass: None,
        }
    }

    pub fn begin_frame(&mut self, device: &Device, surface_texture: &wgpu::TextureView) {
        let mut encoder = device.device()
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        //self.command_encoder = Some(encoder);


        let render_pass = Some(encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: surface_texture,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.2,
                        g: 0.2,
                        b: 0.2,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: surface_texture,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        }));
    }


    // Add methods to interact with the render pass as needed
}
