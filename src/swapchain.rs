use crate::device::Device;
use ash::{vk, Entry};

pub struct Swapchain<'a> {
    swapchain_image_format: Option<vk::Format>,
    swapchain_depth_format: Option<vk::Format>,
    swapchain_extent: Option<vk::Extent2D>,
    swapchain_framebuffers: Option<Vec<vk::Framebuffer>>,
    renderpass: Option<vk::RenderPass>,
    depth_images: Option<Vec<vk::Image>>,
    depth_image_memories: Option<Vec<vk::DeviceMemory>>,
    depth_image_views: Option<Vec<vk::ImageView>>,
    swapchain_images: Option<Vec<vk::Image>>,
    swapchain_image_views: Option<Vec<vk::ImageView>>,
    device: &'a Device<'a>,
    window_extent: Option<vk::Extent2D>,
    swapchain: Option<vk::SwapchainKHR>,
    old_swapchain: Option<vk::SwapchainKHR>,
    image_available_semaphores: Option<Vec<vk::Semaphore>>,
    render_finished_semaphores: Option<Vec<vk::Semaphore>>,
    in_flight_fences: Option<Vec<vk::Fence>>,
    images_in_flight: Option<Vec<vk::Fence>>,
    current_frame: usize,
}

impl<'a> Swapchain<'a> {

    fn new(device:&'a Device<'a>) -> Swapchain<'a>{
        let swapchain = Swapchain::default(&device);

        return swapchain;
    }

    pub fn default(device:&'a Device) -> Swapchain<'a> {
        return Swapchain {
            swapchain_image_format: None,
            swapchain_depth_format: None,
            swapchain_extent: None,
            swapchain_framebuffers: None,
            renderpass: None,
            depth_images: None,
            depth_image_memories: None,
            depth_image_views: None,
            swapchain_images: None,
            swapchain_image_views: None,
            device: device,
            window_extent: None,
            swapchain: None,
            old_swapchain: None,
            image_available_semaphores: None,
            render_finished_semaphores: None,
            in_flight_fences: None,
            images_in_flight: None,
            current_frame: 0,
        };
    }
}
