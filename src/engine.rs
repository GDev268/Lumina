use ash::vk;

use crate::components::camera::Camera;

pub mod device;
pub mod swapchain;
pub mod window;

const MAX_LIGHTS:i32  = 10;

struct PointLight{
    position:glam::Vec4,
    color:glam::Vec4
}

struct GlobalUBO{
    projection:glam::Mat4,
    view:glam::Mat4,
    inverseview:glam::Mat4,
    ambient_light_color:glam::Vec4
}

pub struct FrameInfo<'a>{
    pub frame_index:i32,
    pub frame_time:f64,
    pub command_buffer:vk::CommandBuffer,
    pub camera:&'a Camera,
    pub global_descriptor_set:vk::DescriptorSet,
}