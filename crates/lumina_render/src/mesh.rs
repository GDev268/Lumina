use wgpu::{*, util::{DeviceExt, BufferInitDescriptor}};

use lumina_core::{device::Device, Vertex};
use crate::offset_of;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    has_index_buffer: bool,
    pub index_buffer: Option<wgpu::Buffer>,
    index_count: u32,
}

impl Mesh {
    pub fn new(device: &Device, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let (vertex_buffer, vertex_count) = Mesh::create_vertex_buffer(vertices, device);
        let (index_buffer, has_index_buffer, index_count) =
            Mesh::create_index_buffer(indices, device);

        return Self {
            vertex_buffer: vertex_buffer,
            vertex_count: vertex_count,
            has_index_buffer: has_index_buffer,
            index_buffer: index_buffer,
            index_count: index_count,
        };
    }

    pub fn create_vertex_buffer(vertices: Vec<Vertex>,device: &Device) -> (wgpu::Buffer,u32) {
        let vertex_count = vertices.len() as u32;
        assert!(vertex_count >= 3, "Vertex must be at least 3");

        let buffer = device.device().create_buffer_init(&BufferInitDescriptor{label: Some("Vertex Buffer"),contents: bytemuck::cast_slice(&vertices), usage: wgpu::BufferUsages::VERTEX });

        return (buffer,vertex_count);
    }

    pub fn create_index_buffer(indices: Vec<u32>,device: &Device) -> (Option<wgpu::Buffer>,bool,u32) {
        let index_count = indices.len() as u32;
        let has_index = index_count > 0;

        if !has_index {
            return (None,false,0);
        }
        else{
            let buffer = device.device().create_buffer_init(&BufferInitDescriptor { label: Some("Index Buffer"), contents: bytemuck::cast_slice(&indices), usage: wgpu::BufferUsages::INDEX });

            return(Some(buffer),true,index_count);
        }

    }

    /*pub fn bind(&self, command_buffer: vk::CommandBuffer, device: &Device) {
        let buffers: [vk::Buffer; 1] = [self.vertex_buffer.get_buffer()];
        let offsets: [vk::DeviceSize; 1] = [0];

        unsafe {
            device
                .device()
                .cmd_bind_vertex_buffers(command_buffer, 0, &buffers, &offsets);

            if self.has_index_buffer {
                device.device().cmd_bind_index_buffer(
                    command_buffer,
                    self.index_buffer.as_ref().unwrap().get_buffer(),
                    0,
                    vk::IndexType::UINT32,
                );
            }
        }

    }

    pub fn draw(&self,command_buffer: vk::CommandBuffer,device: &Device) {
        unsafe {
            if self.has_index_buffer {
                device
                    .device()
                    .cmd_draw_indexed(command_buffer, self.index_count, 1, 0, 0, 0);
            } else {
                device
                    .device()
                    .cmd_draw(command_buffer, self.vertex_count, 1, 0, 0);
            }
        }
    }

    pub fn get_attribute_descriptions(&self) -> &Vec<vk::VertexInputAttributeDescription> {
        return &self.attribute_descriptions;
    }

    pub fn get_binding_descriptions(&self) -> &Vec<vk::VertexInputBindingDescription> {
        return &self.binding_descriptions;
    }*/

}
