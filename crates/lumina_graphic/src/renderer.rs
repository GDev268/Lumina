use std::{collections::HashMap, rc::Rc};

use lumina_core::{device::Device, window::Window};

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

    pub fn begin_frame(&mut self, device: &mut Device) {

        //NOTE: CREATE DEPTH_TEXTURE FIELD THAT HAS AN TEXTURE
        
        let output = device.get_surface().get_current_texture().unwrap_or_else(|e| {
            eprintln!("Failed to get surface texture: {:?}", e);
            panic!();
        });

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = device
            .device()
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.2,
                        g: 0.2,
                        b: 0.2,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        drop(render_pass);
        device.queue.submit(std::iter::once(encoder.finish()));
        output.present();

    }
}
