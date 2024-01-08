use std::rc::Rc;

use ash::vk;
use lumina_core::{device::Device, texture::Texture, Vertex2D};
use lumina_graphic::shader::Shader;

use crate::{
    mesh::Mesh,
    offset_of,
    quad::{Quad},
};

#[derive(Clone)]
pub struct Canvas {
    pub mesh: Quad,
    pub shader: Shader,
}

impl Canvas {
    pub fn new(device: Rc<Device>) -> Canvas {
        let mut attribute_descriptions: Vec<vk::VertexInputAttributeDescription> = Vec::new();

        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex2D, position),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Vertex2D, uv),
        });
        
        let mut binding_descriptions: Vec<vk::VertexInputBindingDescription> =
            vec![vk::VertexInputBindingDescription::default()];

        binding_descriptions[0].binding = 0;
        binding_descriptions[0].stride = std::mem::size_of::<Vertex2D>() as u32;
        binding_descriptions[0].input_rate = vk::VertexInputRate::VERTEX;

        Self {
            mesh: Quad::new(Rc::clone(&device)),
            shader: Shader::new(
                Rc::clone(&device),
                "shaders/canvas/canvas_shader.vert",
                "shaders/canvas/canvas_shader.frag",
            ),
        }
    }

    pub fn update(
        &mut self,
        cur_frame: u32,
        extent: vk::Extent2D,
        color_image: vk::Image,
        depth_image: vk::Image,
    ) {
        for (name, _) in self.shader.descriptor_manager.descriptor_table.iter() {
            println!("{:?}", name);
        }

        self.shader
            .descriptor_manager
            .change_image_value_with_images(
                "imageTexture".to_string(),
                cur_frame,
                extent,
                color_image,
                depth_image,
            )
    }

    pub fn render(&mut self, device: &Device, command_buffer: vk::CommandBuffer, cur_frame: u32) {
        unsafe {
            device.device().cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.shader
                    .pipeline
                    .as_ref()
                    .unwrap()
                    .graphics_pipeline
                    .unwrap(),
            );

            device.device().cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.shader.pipeline_layout.unwrap(),
                0,
                &[self.shader.descriptor_manager.get_descriptor_set(cur_frame)],
                &[],
            )
        }
        self.mesh.bind(command_buffer, device);
        self.mesh.draw(command_buffer, device);
    }
}
