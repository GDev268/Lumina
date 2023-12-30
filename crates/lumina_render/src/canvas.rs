use std::rc::Rc;

use ash::vk;
use lumina_core::{device::Device, texture::Texture};
use lumina_graphic::shader::Shader;

use crate::{
    mesh::Mesh,
    offset_of,
    quad::{Quad, Vertex},
};

pub struct Canvas {
    mesh: Quad,
    shader: Shader,
}

impl Canvas {
    pub fn new(device: Rc<Device>) -> Canvas {
        let mut attribute_descriptions: Vec<vk::VertexInputAttributeDescription> = Vec::new();

        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, position),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Vertex, uv),
        });

        let mut binding_descriptions: Vec<vk::VertexInputBindingDescription> =
            vec![vk::VertexInputBindingDescription::default()];

        binding_descriptions[0].binding = 0;
        binding_descriptions[0].stride = std::mem::size_of::<Vertex>() as u32;
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
        framebuffer: vk::Framebuffer,
        cur_frame: u32,
        extent: vk::Extent2D,
        device: &Device,
        color_image: vk::Image,
        depth_image: vk::Image,
    ) {
        self.shader
            .descriptor_manager
            .change_image_value_with_images(
                "ImageTexture".to_string(),
                cur_frame,
                extent,
                color_image,
                depth_image,
            )
    }

    pub fn render(&mut self,device: &Device,command_buffer:vk::CommandBuffer) {
        self.mesh.draw(command_buffer, device);
    }
}
