use crate::device::Device;
use ash::{vk, Entry};

pub struct Swapchain {
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
    window_extent: Option<vk::Extent2D>,
    swapchain: Option<vk::SwapchainKHR>,
    old_swapchain: Option<vk::SwapchainKHR>,
    image_available_semaphores: Option<Vec<vk::Semaphore>>,
    render_finished_semaphores: Option<Vec<vk::Semaphore>>,
    in_flight_fences: Option<Vec<vk::Fence>>,
    images_in_flight: Option<Vec<vk::Fence>>,
    current_frame: usize,
}

impl Swapchain {

    fn new(device:&Device,window_extent:vk::Extent2D) -> Swapchain{
        let swapchain = Swapchain::default();

        return swapchain;
    }

    fn renew(device:&Device,window_extent:vk::Extent2D,previous:Swapchain) -> Swapchain {
        let swapchain = Swapchain::default();

        return swapchain;
    }

    fn init(){

    }

    pub fn default() -> Swapchain {
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

    pub fn get_framebuffer(&self,index:usize) -> vk::Framebuffer{
        return self.swapchain_framebuffers.as_ref().unwrap()[index];
    }

    pub fn get_renderpass(&self) -> vk::RenderPass {
        return self.renderpass.unwrap();
    }

    pub fn get_image_view(&self,index:usize) -> vk::ImageView {
        return self.swapchain_image_views.as_ref().unwrap()[index];
    }

    pub fn image_count(&self) -> usize {
        return self.swapchain_images.as_ref().unwrap().len();
    }

    pub fn get_swapchain_image_format(&self) -> vk::Format {
        return self.swapchain_image_format.unwrap();
    }

    pub fn get_swapchain_extent(&self) -> vk::Extent2D {
        return self.swapchain_extent.unwrap();
    }

    pub fn extent_aspect_ratio(&self) -> f64 {
        return self.swapchain_extent.unwrap().width as f64 / self.swapchain_extent.unwrap().height as f64;
    }

    pub fn find_depth_format()/*-> vk::Format*/{

    } 

    pub fn acquire_next_image(image_index:u32) /*-> vk::Result*/{

    }

    pub fn submit_command_buffers(buffers:vk::CommandBuffer,image_index:u32) /*-> vk::Result*/{

    }

    pub fn compare_swap_formats(&self,swapchain:&Swapchain) -> bool {
        return swapchain.swapchain_depth_format.unwrap() == self.swapchain_depth_format.unwrap() &&
               swapchain.swapchain_image_format.unwrap() == self.swapchain_image_format.unwrap();
        
    }

    fn create_swapchain(self:&mut Swapchain){

    }

    fn create_image_views(self:&mut Swapchain){

    }

    fn create_depth_resources(self:&mut Swapchain){

    }

    fn create_renderpass(self:&mut Swapchain){

    }
    
    fn create_framebuffers(self:&mut Swapchain){

    }

    fn create_sync_objects(self:&mut Swapchain){

    }

    fn choose_swap_surface_format(available_formats:&Vec<vk::SurfaceFormatKHR>)/*  -> vk::SurfaceFormatKHR*/{

    }

    fn choose_swap_present_mode(available_present_modes:&Vec<vk::PresentModeKHR>)/* -> vk::PresentModeKHR */{

    }

    fn choose_swap_extent(surface_capabilites:&vk::SurfaceCapabilitiesKHR)/* -> vk::Extent2D*/ {

    }
}
