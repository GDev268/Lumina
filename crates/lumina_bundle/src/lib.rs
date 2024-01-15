use ash::vk::{self};
use lumina_core::RawLight;

pub struct RendererBundle {
    pub image_format: vk::Format,
    pub depth_format: vk::Format,
    pub max_extent: vk::Extent2D,
    pub render_pass: vk::RenderPass,
    pub wait_semaphore: vk::Semaphore
}

pub struct ResourcesBundle {
    pub command_buffer: vk::CommandBuffer,
    pub raw_lights:Vec<RawLight>,
    pub cur_frame: u32,
    pub cur_projection: [[f32;4];4],
    pub cur_render_pass:vk::RenderPass,
}

impl Default for ResourcesBundle {
    fn default() -> Self {
        Self {
            command_buffer: vk::CommandBuffer::null(),
            raw_lights: Vec::new(),
            cur_frame: 0,
            cur_projection: [[0.0;4];4],
            cur_render_pass: vk::RenderPass::null()
        }
    }
}
