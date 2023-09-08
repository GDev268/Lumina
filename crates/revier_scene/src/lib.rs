pub mod scene;

use ash::vk;
use revier_render::camera::Camera;

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
    pub frame_time:f64,
    pub command_buffer:vk::CommandBuffer,
    pub camera:&'a Camera,
    //pub global_descriptor_set:vk::DescriptorSet,
}