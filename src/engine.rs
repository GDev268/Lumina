use ash::vk;

pub mod device;
pub mod swapchain;
pub mod window;
pub mod model;
pub mod game_object;

const MAX_LIGHTS:i32  = 10;

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            (std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize).try_into().unwrap()
        }
    }};
}

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