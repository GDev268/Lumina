/*use lumina_core::{device::Device, Vertex};
//use lumina_render::{mesh::Vertex, offset_of};

use ash::vk::{self};
use std::{ffi::CString, num::NonZeroU32};

use crate::{pipeline, shader::Shader};

#[derive(Debug)]
pub struct PipelineConfiguration {
    pub primitive: wgpu::PrimitiveState,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    pub multisample: wgpu::MultisampleState,
    pub multiview: Option<NonZeroU32>,
    pub pipeline_layout: Option<wgpu::PipelineLayout>,
}

impl Default for PipelineConfiguration {
    fn default() -> Self {
        let primitive = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        };

        let depth_stencil = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });

        let multisample = wgpu::MultisampleState {
            count: 1,
            mask: 10,
            alpha_to_coverage_enabled: false,
        };


        Self {
            primitive,
            depth_stencil: None,
            multisample,
            multiview: None,
            pipeline_layout: None,
        }
    }
}

pub struct Pipeline {
    pub graphics_pipeline: Option<wgpu::RenderPipeline>,
}

impl Pipeline {
    pub fn new(
        device: &Device,
        shader: &wgpu::ShaderModule,
        pipeline_config: &PipelineConfiguration,
        pipeline_id: &str,
    ) -> Self {

        let mut pipeline = Pipeline::default();

        pipeline.graphics_pipeline = Some(device.device().create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some(&format!("{}_Pipeline", pipeline_id)),
                layout: pipeline_config.pipeline_layout.as_ref(),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::description()],
                },
                primitive: pipeline_config.primitive,
                depth_stencil: None,
                multisample: pipeline_config.multisample,
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: device.get_surface_format(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: pipeline_config.multiview,
            },
        ));

        return pipeline;
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        return Self {
            graphics_pipeline: None,
        };
    }
}
*/