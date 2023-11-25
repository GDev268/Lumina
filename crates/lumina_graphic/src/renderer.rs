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
    depth_texture: wgpu::Texture,
}

impl<'a> Renderer<'a> {
    pub fn new(device: &Device,window: &Window) -> Self {
        let depth_texture = device.device().create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: window.width,
                height: window.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Depth32Float],
        });

        Self {
            command_encoder: None,
            render_pass: None,
            depth_texture
        }
    }

    pub fn begin_frame(&mut self, device: &mut Device) {
        //NOTE: CREATE DEPTH_TEXTURE FIELD THAT HAS AN TEXTURE

        let output = device
            .get_surface()
            .get_current_texture()
            .unwrap_or_else(|e| {
                eprintln!("Failed to get surface texture: {:?}", e);
                panic!();
            });

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let depth_view = self.depth_texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = device
            .device()
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
                view: &depth_view,
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

    pub fn update(&mut self,device: &Device,window: &Window){
        if (self.depth_texture.size().width,self.depth_texture.size().height) != (window.width,window.height) {
            self.depth_texture = device.device().create_texture(&TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: window.width,
                    height: window.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[wgpu::TextureFormat::Depth32Float],
            });
        }
    }
}
