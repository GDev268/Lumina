use ash::vk::{self,};

pub struct RendererBundle{
    pub image_format:vk::Format,
    pub depth_format:vk::Format,
    pub max_extent:vk::Extent2D,
    pub render_pass:vk::RenderPass
}

pub struct ResourcesBundle {
    pub command_buffer:vk::CommandBuffer,
}