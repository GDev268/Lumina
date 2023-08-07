use ash::vk;

pub mod device;
pub mod swapchain;
pub mod window;

const MAX_LIGHTS:i32  = 10;

struct PointLight{
    position:glam::Vec4,
    color:glam::Vec4
}

struct GlobalUbo{
    projection:glam::Mat4,
    view:glam::Mat4,
    inverseview:glam::Mat4,
    ambient_light_color:glam::Vec4
}

struct FrameInfo{
    frame_index:i32,
    frame_time:f64,
    command_buffer:vk::CommandBuffer,
    global_descriptor_set:vk::DescriptorSet,
}