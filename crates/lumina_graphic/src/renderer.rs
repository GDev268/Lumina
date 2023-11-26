use std::{collections::HashMap, rc::Rc, cell::RefCell};

use lumina_core::{device::Device, window::Window};

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

use crate::{pipeline::{Pipeline, PipelineConfiguration}, shader::Shader};

pub struct Renderer<'a> {
    device: Rc<RefCell<Device>>,
    command_encoder: Option<wgpu::CommandEncoder>,
    render_pass: Option<wgpu::RenderPass<'a>>,
    depth_texture: wgpu::Texture,
    cur_pipeline:Option<wgpu::RenderPipeline>
}

impl<'a> Renderer<'a> {
    pub fn new(device: Rc<RefCell<Device>>,window: &Window) -> Self {
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
            command_encoder: None,
            render_pass: None,
            depth_texture,
            cur_pipeline: None
        }
    }

    pub fn begin_frame(&mut self) {
        //NOTE: CREATE DEPTH_TEXTURE FIELD THAT HAS AN TEXTURE

        let output = self.device.borrow()
            .get_surface()
            .get_current_texture().unwrap();
                  

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let depth_view = self.depth_texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.borrow_mut()
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
        self.device.borrow_mut().queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
    }

    pub async fn render_object(&'a mut self,shader:&Shader,mesh:&'a Mesh) {
        let pipeline = Pipeline::new(&self.device.borrow(),shader,&PipelineConfiguration::default(),"0");

        self.cur_pipeline = pipeline.graphics_pipeline;
        self.render_pass.as_mut().unwrap().set_pipeline(&self.cur_pipeline.as_ref().unwrap());
        self.render_pass.as_mut().unwrap().set_vertex_buffer(0,mesh.vertex_buffer.slice(..));
        self.render_pass.as_mut().unwrap().draw(0..mesh.vertex_count, 0..0);

    }

    pub fn update(&mut self, window: &Window){
        if (self.depth_texture.size().width,self.depth_texture.size().height) != (window.width,window.height) {
            self.depth_texture = self.device.borrow().device().create_texture(&TextureDescriptor {
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
