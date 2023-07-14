use crate::device::{Device, QueueFamily, SwapChainSupportDetails};
use ash::{
    vk::{self, TaggedStructure},
    Entry,
};
use std::ptr::{self};

struct SwapchainKHR {
    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
}

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
    swapchain: Option<SwapchainKHR>,
    image_available_semaphores: Option<Vec<vk::Semaphore>>,
    render_finished_semaphores: Option<Vec<vk::Semaphore>>,
    in_flight_fences: Option<Vec<vk::Fence>>,
    images_in_flight: Option<Vec<vk::Fence>>,
    current_frame: usize,
}

impl Swapchain {
    fn new(device: &Device, window_extent: vk::Extent2D) -> Swapchain {
        let mut swapchain = Swapchain::default();
        Swapchain::init(&mut swapchain, None, device);

        return swapchain;
    }

    fn renew(device: &Device, window_extent: vk::Extent2D, previous: &mut Swapchain) -> Swapchain {
        let mut swapchain = Swapchain::default();

        Swapchain::init(&mut swapchain, Some(previous), device);

        return swapchain;
    }

    fn init(self: &mut Swapchain, old_swapchain: Option<&mut Swapchain>, device: &Device) {
        Swapchain::create_swapchain(
            self,
            device,
            Some(&mut old_swapchain.unwrap().swapchain.as_ref().unwrap()),
        );
        Swapchain::create_image_views(self);
        Swapchain::create_renderpass(self);
        Swapchain::create_depth_resources(self);
        Swapchain::create_framebuffers(self);
        Swapchain::create_sync_objects(self);
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
            image_available_semaphores: None,
            render_finished_semaphores: None,
            in_flight_fences: None,
            images_in_flight: None,
            current_frame: 0,
        };
    }

    pub fn get_framebuffer(&self, index: usize) -> vk::Framebuffer {
        return self.swapchain_framebuffers.as_ref().unwrap()[index];
    }

    pub fn get_renderpass(&self) -> vk::RenderPass {
        return self.renderpass.unwrap();
    }

    pub fn get_image_view(&self, index: usize) -> vk::ImageView {
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
        return self.swapchain_extent.unwrap().width as f64
            / self.swapchain_extent.unwrap().height as f64;
    }

    pub fn find_depth_format() /*-> vk::Format*/ {}

    pub fn acquire_next_image(image_index: u32) /*-> vk::Result*/ {}

    pub fn submit_command_buffers(buffers: vk::CommandBuffer, image_index: u32) /*-> vk::Result*/ {}

    pub fn compare_swap_formats(&self, swapchain: &Swapchain) -> bool {
        return swapchain.swapchain_depth_format.unwrap() == self.swapchain_depth_format.unwrap()
            && swapchain.swapchain_image_format.unwrap() == self.swapchain_image_format.unwrap();
    }

    fn create_swapchain(
        self: &mut Swapchain,
        device: &Device,
        old_swapchain: Option<&SwapchainKHR>,
    ) {
        let swapchain_support: SwapChainSupportDetails = device.get_swapchain_support();

        let surface_format: vk::SurfaceFormatKHR = self
            .choose_swap_surface_format(&swapchain_support.surface_formats.unwrap())
            .unwrap();
        let present_mode: vk::PresentModeKHR = self
            .choose_swap_present_mode(&swapchain_support.present_modes.unwrap())
            .unwrap();
        let extent: vk::Extent2D =
            self.choose_swap_extent(&swapchain_support.surface_capabilities.unwrap());

        let mut image_count: u32 = swapchain_support
            .surface_capabilities
            .unwrap()
            .min_image_count
            + 1;
        if swapchain_support
            .surface_capabilities
            .unwrap()
            .max_image_count
            > 0
            && image_count
                > swapchain_support
                    .surface_capabilities
                    .unwrap()
                    .max_image_count
        {
            image_count = swapchain_support
                .surface_capabilities
                .unwrap()
                .max_image_count;
        }

        let mut create_info: vk::SwapchainCreateInfoKHR = vk::SwapchainCreateInfoKHR::default();

        self.swapchain.as_mut().unwrap().swapchain_loader = ash::extensions::khr::Swapchain::new(
            &device.instance.as_ref().unwrap(),
            &device.device(),
        );

        create_info.s_type = vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR;
        create_info.surface = device.surface();
        create_info.min_image_count = image_count;
        create_info.image_format = surface_format.format;
        create_info.image_color_space = surface_format.color_space;
        create_info.image_extent = extent;
        create_info.image_array_layers = 1;
        create_info.image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;

        let indices: QueueFamily = device.find_physical_queue_families();
        let queue_family_indices: [u32; 2] = [indices.graphics_family, indices.present_family];

        if indices.graphics_family != indices.present_family {
            create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
            create_info.queue_family_index_count = 2;
            create_info.p_queue_family_indices = queue_family_indices.as_ptr();
        } else {
            create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
            create_info.queue_family_index_count = 0;
            create_info.p_queue_family_indices = ptr::null();
        }

        create_info.pre_transform = swapchain_support
            .surface_capabilities
            .unwrap()
            .current_transform;
        create_info.composite_alpha = vk::CompositeAlphaFlagsKHR::OPAQUE;
        create_info.present_mode = present_mode;
        create_info.clipped = vk::TRUE;

        if old_swapchain.is_none() {
            create_info.old_swapchain = self.swapchain.as_ref().unwrap().swapchain;
        } else {
            create_info.old_swapchain = old_swapchain.unwrap().swapchain;
        }

        unsafe {
            self.swapchain.as_mut().unwrap().swapchain = self
                .swapchain
                .as_ref()
                .unwrap()
                .swapchain_loader
                .create_swapchain(&create_info, None)
                .expect("Failed to create swapchain!");
        }
    }

    fn create_image_views(self: &mut Swapchain) {}

    fn create_depth_resources(self: &mut Swapchain) {}

    fn create_renderpass(self: &mut Swapchain) {}

    fn create_framebuffers(self: &mut Swapchain) {}

    fn create_sync_objects(self: &mut Swapchain) {}

    fn choose_swap_surface_format(
        &self,
        available_formats: &Vec<vk::SurfaceFormatKHR>,
    ) -> Option<vk::SurfaceFormatKHR> {
        for available_format in available_formats {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return Some(*available_format);
            }
        }
        return None;
    }

    fn choose_swap_present_mode(
        &self,
        available_present_modes: &Vec<vk::PresentModeKHR>,
    ) -> Option<vk::PresentModeKHR> {
        for present_mode in available_present_modes {
            if *present_mode == vk::PresentModeKHR::MAILBOX {
                println!("Present mode: Mailbox");
                return Some(*present_mode);
            }
        }

        return None;
    }

    fn choose_swap_extent(&self, surface_capabilites: &vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if surface_capabilites.current_extent.width != u32::MAX {
            return surface_capabilites.current_extent;
        } else {
            let mut actual_extent: vk::Extent2D = self.window_extent.unwrap();
            actual_extent.width = std::cmp::max(
                surface_capabilites.min_image_extent.width,
                std::cmp::min(
                    surface_capabilites.max_image_extent.width,
                    actual_extent.width,
                ),
            );

            actual_extent.height = std::cmp::max(
                surface_capabilites.min_image_extent.height,
                std::cmp::min(
                    surface_capabilites.max_image_extent.height,
                    actual_extent.height,
                ),
            );

            return actual_extent;
        }
    }
}
