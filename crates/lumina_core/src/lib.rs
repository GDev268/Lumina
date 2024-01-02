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