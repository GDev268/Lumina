use std::any::Any;

pub mod device;
pub mod window;
pub mod swapchain;
pub mod image;
pub mod framebuffer;
pub mod fps_manager;
pub mod texture;

pub trait BufferValue: Any {}

pub trait ImageValue: Any {}