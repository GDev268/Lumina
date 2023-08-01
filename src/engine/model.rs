use ash::vk;

use crate::data::buffer::Buffer;
use crate::engine::device::Device;
use crate::offset_of;

struct Vertex {
    position: glam::Vec3,
    color: glam::Vec3,
    normal: glam::Vec3,
    uv: glam::Vec2,
}

struct Builder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Builder {
    pub fn load_model(file_path: &str) {}
}

struct Model {
    vertex_buffer: Buffer,
    vertex_count: u32,
    has_index_buffer: bool,
    index_buffer: Buffer,
    index_count: u32,
    binding_descriptions: Vec<vk::VertexInputBindingDescription>,
    attribute_descriptions: Vec<vk::VertexInputAttributeDescription>,
}

impl Model {
    pub fn new(device: &Device, builder: Builder) /*-> Self*/
    {
        let (attributes, bindings) = Model::setup();

        let vertex_buffer = Model::create_vertex_buffers(builder.vertices);
        let index_buffer = Model::create_index_buffers(builder.indices);
    }

    pub fn create_model_from_file(device: &Device, file_path: &str) {}

    pub fn bind(command_buffer: vk::CommandBuffer) {}

    pub fn draw(command_buffer: vk::CommandBuffer) {}

    pub fn get_attribute_descriptions(&self) -> &Vec<vk::VertexInputAttributeDescription> {
        return &self.attribute_descriptions;
    }

    pub fn get_binding_descriptions(&self) -> &Vec<vk::VertexInputBindingDescription> {
        return &self.binding_descriptions;
    }

    fn create_vertex_buffers(vertices: Vec<Vertex>, device: &Device) {
        let vertex_count = vertices.len() as u32;
        assert!(vertex_count >= 3, "Vertex must be at least 3");
        let buffer_size: vk::DeviceSize =
            (std::mem::size_of::<Vertex>() * vertex_count as usize) as u64;
        let vertex_size = std::mem::size_of::<Vertex>() as u64;

        let staging_buffer: Buffer = Buffer::new(
            device,
            vertex_size,
            vertex_count,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            None,
        );

        staging_buffer.map(device, size, offset)
    }

    fn create_index_buffers(indices: Vec<u32>) {}

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
            offset: offset_of!(Vertex, color),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, normal),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 3,
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
