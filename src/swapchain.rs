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
        Swapchain::create_image_views(self, device);
        Swapchain::create_renderpass(self, device);
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

    pub fn find_depth_format(&self, device: &Device) -> vk::Format {
        return device.find_support_format(
            &vec![
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        );
    }

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
            create_info.queue_family_index_count = 0;
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

            self.swapchain_images = Some(
                self.swapchain
                    .as_ref()
                    .unwrap()
                    .swapchain_loader
                    .get_swapchain_images(self.swapchain.as_ref().unwrap().swapchain)
                    .unwrap(),
            );

            self.swapchain_image_format = Some(surface_format.format);
            self.swapchain_extent = Some(extent);
        }
    }

    fn create_image_views(self: &mut Swapchain, device: &Device) {
        for i in 0..self.swapchain_images.as_ref().unwrap().len() {
            let mut view_info: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo::default();
            view_info.s_type = vk::StructureType::IMAGE_VIEW_CREATE_INFO;
            view_info.image = self.swapchain_images.as_ref().unwrap()[i];
            view_info.view_type = vk::ImageViewType::TYPE_2D;
            view_info.format = self.swapchain_image_format.unwrap();
            view_info.subresource_range.aspect_mask = vk::ImageAspectFlags::COLOR;
            view_info.subresource_range.base_mip_level = 0;
            view_info.subresource_range.level_count = 1;
            view_info.subresource_range.base_array_layer = 0;
            view_info.subresource_range.layer_count = 1;

            unsafe {
                self.swapchain_image_views.as_mut().unwrap()[i] = device
                    .device()
                    .create_image_view(&view_info, None)
                    .expect("Failed to create image view!");
            }
        }
    }

    fn create_renderpass(self: &mut Swapchain, device: &Device) {
        let mut depth_attachment: vk::AttachmentDescription = vk::AttachmentDescription::default();
        depth_attachment.format = self.find_depth_format(device);
        depth_attachment.samples = vk::SampleCountFlags::TYPE_1;
        depth_attachment.load_op = vk::AttachmentLoadOp::CLEAR;
        depth_attachment.store_op = vk::AttachmentStoreOp::DONT_CARE;
        depth_attachment.stencil_load_op = vk::AttachmentLoadOp::DONT_CARE;
        depth_attachment.stencil_store_op = vk::AttachmentStoreOp::DONT_CARE;
        depth_attachment.initial_layout = vk::ImageLayout::UNDEFINED;
        depth_attachment.final_layout = vk::ImageLayout::ATTACHMENT_OPTIMAL;

        let mut depth_attachment_ref: vk::AttachmentReference = vk::AttachmentReference::default();
        depth_attachment_ref.attachment = 1;
        depth_attachment_ref.layout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;

        let mut color_attachment: vk::AttachmentDescription = vk::AttachmentDescription::default();
        color_attachment.format = self.get_swapchain_image_format();
        color_attachment.samples = vk::SampleCountFlags::TYPE_1;
        depth_attachment.load_op = vk::AttachmentLoadOp::CLEAR;
        depth_attachment.store_op = vk::AttachmentStoreOp::STORE;
        depth_attachment.stencil_load_op = vk::AttachmentLoadOp::DONT_CARE;
        depth_attachment.stencil_store_op = vk::AttachmentStoreOp::DONT_CARE;
        depth_attachment.initial_layout = vk::ImageLayout::UNDEFINED;
        depth_attachment.final_layout = vk::ImageLayout::PRESENT_SRC_KHR;

        let mut color_attachment_ref: vk::AttachmentReference = vk::AttachmentReference::default();
        depth_attachment_ref.attachment = 1;
        depth_attachment_ref.layout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;

        let mut subpass_description:vk::SubpassDescription = vk::SubpassDescription::default();
        subpass_description.pipeline_bind_point = vk::PipelineBindPoint::GRAPHICS;
        
    }

    fn create_depth_resources(self: &mut Swapchain) {}

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
