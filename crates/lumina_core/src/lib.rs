use std::any::Any;

pub mod device;
pub mod window;
pub mod swapchain;
pub mod image;
pub mod framebuffer;
pub mod fps_manager;
pub mod texture;

#[derive(Clone, Copy)]
pub struct Vertex3D {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub uv: glam::Vec2,
}

#[derive(Clone, Copy)]
pub struct Vertex2D {
    pub position: glam::Vec3,
    pub uv: glam::Vec2,
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
    pub intensity: f32,

    pub range: f32,
    pub spot_size: f32,
    pub light_type: u32,
}


impl Default for RawLight {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            color: [0.0;3],
            rotation: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            spot_size: 0.0,
            light_type: 0,
            _padding1: 0,
            _padding2: 0,
        }
    }
}
