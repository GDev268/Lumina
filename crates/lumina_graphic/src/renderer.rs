/*use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    pipeline::{Pipeline, PipelineConfiguration},
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

    pub fn render_object(&self, shader: &Shader, mesh: &Mesh) {
        let vertices: Vec<Vertex> = vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                normal: [0.0, 0.0, 1.0],  // Assuming a normal pointing out of the 2D plane
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                normal: [0.0, 0.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                normal: [0.0, 0.0, 1.0],
                uv: [0.0, 1.0],
            },
        ];

        
        let pipe_config = &mut PipelineConfiguration::default();

        pipe_config.pipeline_layout = Some(self.device.borrow().device().create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            },
        ));

        /*let pipeline = Pipeline::new(&self.device.borrow(), shader, &pipe_config, "0");

        //NOTE: CREATE DEPTH_TEXTURE FIELD THAT HAS AN TEXTURE

        let output = self
            .device
            .borrow()
            .get_surface()
            .get_current_texture()
            .unwrap();

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let depth_view = self
            .depth_texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder =
            self.device
                .borrow_mut()
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
            depth_stencil_attachment: None
        });

        render_pass.set_pipeline(&pipeline.graphics_pipeline.as_ref().unwrap());
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.draw(0..vertices.len() as u32, 0..1);

        drop(render_pass);
        self.device
            .borrow_mut()
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();*/
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
*/