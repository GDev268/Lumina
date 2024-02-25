use std::any::Any;
use ash::*;

pub mod device;
pub mod window;
pub mod swapchain;
pub mod image;
pub mod framebuffer;
pub mod fps_manager;
pub mod texture;

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        unsafe {
            let b: $base = std::mem::zeroed();
            (std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize).try_into().unwrap()
        }
    }};
}


#[derive(Clone, Copy)]
pub struct Vertex3D {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub uv: glam::Vec2,
}

impl Vertex3D {
    pub fn setup() -> (
        Vec<vk::VertexInputAttributeDescription>,
        Vec<vk::VertexInputBindingDescription>,
    ) {
        let mut attribute_descriptions: Vec<vk::VertexInputAttributeDescription> = Vec::new();

        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Self, position),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Self, normal),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, uv),
        });

        let mut binding_descriptions: Vec<vk::VertexInputBindingDescription> =
            vec![vk::VertexInputBindingDescription::default()];

        binding_descriptions[0].binding = 0;
        binding_descriptions[0].stride = std::mem::size_of::<Vertex3D>() as u32;
        binding_descriptions[0].input_rate = vk::VertexInputRate::VERTEX;

        return (attribute_descriptions, binding_descriptions);
    }
}

#[derive(Clone, Copy)]
pub struct Vertex2D {
    pub position: glam::Vec2,
    pub color: glam::Vec4,
    pub uv: glam::Vec2,
}

impl Vertex2D {
    pub fn setup() -> (
        Vec<vk::VertexInputAttributeDescription>,
        Vec<vk::VertexInputBindingDescription>,
    ) {
        let mut attribute_descriptions: Vec<vk::VertexInputAttributeDescription> = Vec::new();

        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, position),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(Self, color),
        });
        attribute_descriptions.push(vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, uv),
        });

        let mut binding_descriptions: Vec<vk::VertexInputBindingDescription> =
            vec![vk::VertexInputBindingDescription::default()];

        binding_descriptions[0].binding = 0;
        binding_descriptions[0].stride = std::mem::size_of::<Vertex2D>() as u32;
        binding_descriptions[0].input_rate = vk::VertexInputRate::VERTEX;

        return (attribute_descriptions, binding_descriptions);
    }
}


pub trait BufferValue: Any {}

pub trait ImageValue: Any {}

#[repr(C,align(16))]
#[derive(Debug, Clone, Copy)]
pub struct RawLight {
    pub position: [f32; 3],
    pub _padding1: u32,

    pub color: [f32; 3],
    pub _padding2: u32,
    
    pub rotation: [f32; 3],
    pub _padding3: u32,

    pub intensity: f32,
    pub spot_size: f32,

    pub linear:f32,
    pub quadratic:f32,

    pub light_type: u32,
}


impl Default for RawLight {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            color: [0.0;3],
            rotation: [0.0; 3],
            intensity: 0.0,
            spot_size: 0.0,
            light_type: 0,
            linear: 0.0,
            quadratic: 0.0,
            _padding1: 0,
            _padding2: 0,
            _padding3: 0,
        }
    }
}
