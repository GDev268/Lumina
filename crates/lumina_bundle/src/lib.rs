use ash::vk;

pub struct CameraBundle{
    pub image_format:vk::Format,
    pub depth_format:vk::Format,
    pub max_extent:vk::Extent2D,
    pub render_pass:vk::RenderPass
}

struct GlobalBundles {
    camera:CameraBundle
}