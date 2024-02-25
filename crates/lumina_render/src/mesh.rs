use std::{rc::Rc, sync::Arc};

use ash::vk;

use lumina_data::buffer::Buffer;
use lumina_core::device::Device;
use crate::offset_of;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub uv: glam::Vec2,
}

pub struct Mesh {
    vertex_buffer: Buffer,
    vertex_count: u32,
    has_index_buffer: bool,
    index_buffer: Option<Buffer>,
    index_count: u32,
    binding_descriptions: Vec<vk::VertexInputBindingDescription>,
    attribute_descriptions: Vec<vk::VertexInputAttributeDescription>,
    vertex_array:Vec<Vertex>,
    index_array:Vec<u32>
}

impl Mesh {
    pub fn new(device: Arc<Device>, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let (attributes, bindings) = Mesh::setup();

        let (vertex_buffer, vertex_count) = Mesh::create_vertex_buffers(vertices.clone(), Arc::clone(&device));
        let (index_count, has_index_buffer, index_buffer) =
            Mesh::create_index_buffers(indices.clone(), Arc::clone(&device));

        return Self {
            vertex_buffer: vertex_buffer,
            vertex_count: vertex_count,
            has_index_buffer: has_index_buffer,
            index_buffer: index_buffer,
            index_count: index_count,
            binding_descriptions: bindings,
            attribute_descriptions: attributes,
            vertex_array: vertices,
            index_array: indices
        };
    }

    pub fn bind(&self, command_buffer: vk::CommandBuffer, device: &Device) {
        let buffers: [vk::Buffer; 1] = [self.vertex_buffer.get_buffer()];
        let offsets: [vk::DeviceSize; 1] = [0];

        unsafe {
            device.device().device_wait_idle().unwrap();
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
            device.device().device_wait_idle().unwrap();
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
    }

    fn create_vertex_buffers(vertices: Vec<Vertex>, device: Arc<Device>) -> (Buffer, u32) {
        let vertex_count = vertices.len() as u32;
        assert!(vertex_count >= 3, "Vertex must be at least 3");
        let buffer_size: vk::DeviceSize =
            (std::mem::size_of::<Vertex>() * vertex_count as usize) as u64;
        let vertex_size = std::mem::size_of::<Vertex>() as vk::DeviceSize;

        let mut staging_buffer: Buffer = Buffer::new(
            Arc::clone(&device),
            vertex_size,
            vertex_count as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,

        );

        staging_buffer.map(None,None);
        staging_buffer.write_to_buffer(&vertices,None,None);

        let vertex_buffer = Buffer::new(
            Arc::clone(&device),
            vertex_size,
            vertex_count as u64,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );

        device.copy_buffer(
            staging_buffer.get_buffer(),
            vertex_buffer.get_buffer(),
            buffer_size,
        );

        return (vertex_buffer, vertex_count);
    }

    fn create_index_buffers(indices: Vec<u32>, device: Arc<Device>) -> (u32, bool, Option<Buffer>) {
        let index_count = indices.len() as u32;
        let has_index_buffer = index_count > 0;

        if !has_index_buffer {
            return (0, false, None);
        }

        let buffer_size: vk::DeviceSize =
            (std::mem::size_of::<u32>() * index_count as usize) as u64;
        let index_size = std::mem::size_of::<u32>() as u64;

        let mut staging_buffer = Buffer::new(
            Arc::clone(&device),
            index_size,
            index_count as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        staging_buffer.map(None,None);
        staging_buffer.write_to_buffer(&indices,None,None);

        let index_buffer = Buffer::new(
            Arc::clone(&device),
            index_size,
            index_count as u64,
            vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );

        device.copy_buffer(
            staging_buffer.get_buffer(),
            index_buffer.get_buffer(),
            buffer_size,
        );

        return (index_count, has_index_buffer, Some(index_buffer));
    }

    fn setup() -> (
        Vec<vk::VertexInputAttributeDescription>,
        Vec<vk::VertexInputBindingDescription>,
    ) {
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
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, normal),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Vertex, uv),
        });

        let mut binding_descriptions: Vec<vk::VertexInputBindingDescription> =
            vec![vk::VertexInputBindingDescription::default()];

        binding_descriptions[0].binding = 0;
        binding_descriptions[0].stride = std::mem::size_of::<Vertex>() as u32;
        binding_descriptions[0].input_rate = vk::VertexInputRate::VERTEX;

        return (attribute_descriptions, binding_descriptions);
    }
}
