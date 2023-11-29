use std::{cell::RefCell, collections::HashMap, rc::Rc, iter};

use lumina_core::{device::Device, window::Window, Vertex};

//use lumina_geometry::model::{Model, PushConstantData};
use lumina_object::{game_object::GameObject, transform::Transform};
use lumina_render::mesh::Mesh;
use lumina_scene::query::Query;

use wgpu::*;

/*use crate::shader::FieldData;

use super::{
    pipeline::{Pipeline, PipelineConfiguration},
    shader::Shader,
    types::{LuminaShaderType, LuminaShaderTypeConverter},
};*/

use std::fmt::Debug;

use crate::{
    pipeline::{Pipeline, PipelineConfiguration, self},
    shader::Shader,
};

pub struct Renderer {
    device: Rc<RefCell<Device>>,
    depth_texture: wgpu::Texture,
}

impl Renderer {
    pub fn new(device: Rc<RefCell<Device>>, window: &Window) -> Self {
        let depth_texture = device.borrow().device().create_texture(&TextureDescriptor {
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
            device,
            depth_texture,
        }
    }

    pub fn render_object(&self, shader: &Shader, mesh: &Mesh, pipeline:&Pipeline) {
        let output = self.device.borrow().get_surface().get_current_texture().expect("Faield to get the surface");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device.borrow().device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&pipeline.graphics_pipeline);

            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..9 , 0, 0..1);
        }

        self.device.borrow().queue.submit(iter::once(encoder.finish()));
        output.present();
    }

    pub fn update(&mut self, window: &Window) {
        if (
            self.depth_texture.size().width,
            self.depth_texture.size().height,
        ) != (window.width, window.height)
        {
            self.depth_texture = self
                .device
                .borrow()
                .device()
                .create_texture(&TextureDescriptor {
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
