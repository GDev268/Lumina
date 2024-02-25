use std::{collections::HashMap, hash::Hash, rc::Rc, sync::Arc};

use ash::vk;

use image::DynamicImage;
use lumina_atlas::atlas::Atlas;
use lumina_core::{device::Device, texture::Texture, RawLight, Vertex2D};
use lumina_data::buffer::Buffer;
use lumina_graphic::shader::Shader;
use lumina_object::game_object::{Component, GameObject};
use lumina_pbr::material::Material;
use russimp::scene::{PostProcess, Scene};
use serde_json::Value;

use crate::{
    camera::Camera,
    mesh::{Mesh, Vertex},
};

pub struct GuiCanvas {
    device: Arc<Device>,
    pub materials: Vec<Material>,
    pub mesh_material_bindings: HashMap<usize, usize>,
    pub shader: Shader,
    pub atlas: Atlas,
}

impl GuiCanvas {
    pub fn new(device: Arc<Device>,render_pass:vk::RenderPass) -> Self {
        let mut shader = Shader::new(
            Arc::clone(&device),
            "shaders/gui/gui_shader.vert",
            "shaders/gui/gui_shader.frag",
            lumina_core::Vertex2D::setup(),
        );

        shader.create_pipeline_layout(false);
        shader.create_pipeline(render_pass);

        let mut mesh_material_bindings: HashMap<usize, usize> = HashMap::new();
        mesh_material_bindings.insert(0, 0);

        Self {
            device: Arc::clone(&device),
            mesh_material_bindings,
            materials: Vec::new(),
            shader,
            atlas: Atlas::new(),
        }
    }

    pub fn add_texture(&mut self, data: Vec<[u8; 4]>, width: u32, height: u32) -> Texture {
        let mut texture = Texture::new("");

        self.atlas
            .pack_from_bytes(vec![data], width, height, vec![&mut texture]);

        texture
    }

    pub fn render(
        &mut self,
        command_buffer: vk::CommandBuffer,
        vertices: Vec<Vec<Vertex2D>>,
        indices: Vec<Vec<u32>>,
        frame_index: u32,
        gui_camera: Camera,
    ) {
        for i in 0..vertices.len() {
            let vertex_count = vertices[i].len() as u32;
            assert!(vertex_count >= 3, "Vertex must be at least 3");
            let buffer_size: vk::DeviceSize =
                (std::mem::size_of::<Vertex2D>() * vertex_count as usize) as u64;
            let vertex_size = std::mem::size_of::<Vertex2D>() as vk::DeviceSize;

            let mut vertex_buffer = Buffer::new(
                Arc::clone(&self.device),
                vertex_size,
                vertex_count as u64,
                vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );

            vertex_buffer.map(None, None);
            vertex_buffer.write_to_buffer(&vertices[i], None, None);

            let index_count = indices[i].len() as u32;
            let index_size = std::mem::size_of::<u32>() as u64;

            let mut index_buffer = Buffer::new(
                Arc::clone(&self.device),
                index_size,
                index_count as u64,
                vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );

            index_buffer.map(None, None);
            index_buffer.write_to_buffer(&indices[i], None, None);

            unsafe {
                self.shader.pipeline.as_ref().unwrap().bind(&self.device, command_buffer);

                self.device.device().device_wait_idle().unwrap();

                self.device.device().cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[vertex_buffer.get_buffer()],
                    &[0],
                );

                self.device.device().cmd_bind_index_buffer(
                    command_buffer,
                    index_buffer.get_buffer(),
                    0,
                    vk::IndexType::UINT32,
                );

                self.device.device().device_wait_idle().unwrap();

            }

            /*let material = &self.materials[*self.mesh_material_bindings.get(&id).unwrap()];

            unsafe {
                self.device.device().cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.shader.pipeline_layout.unwrap(),
                    0,
                    &[self
                        .shader
                        .descriptor_manager
                        .get_descriptor_set(frame_index as u32)],
                    &[],
                );
            }

            mesh.bind(command_buffer, device);
            mesh.draw(command_buffer, device);´*/
        }
    }
}
