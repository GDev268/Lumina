use crate::device::{Device, QueueFamily, SwapChainSupportDetails};
use crate::framebuffer::{Framebuffer, self};
use crate::image::Image;
use ash::vk::{self};
use std::ptr::{self};

pub const MAX_FRAMES_IN_FLIGHT: usize = 3;

struct SwapchainKHR {
    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
}

pub struct Swapchain {
    swapchain_image_format: Option<vk::Format>,
    swapchain_depth_format: Option<vk::Format>,
    swapchain_extent: Option<vk::Extent2D>,
    swapchain_framebuffers: Vec<Framebuffer>,
    renderpass: Option<vk::RenderPass>,
    depth_images: Vec<Image>,
    swapchain_images: Vec<Image>,
    window_extent: vk::Extent2D,
    swapchain: Option<SwapchainKHR>,
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,
    pub current_frame: usize,
}

impl Swapchain {
    pub fn new(device: &Device, window_extent: vk::Extent2D) -> Swapchain {
        let mut swapchain = Swapchain::default();
        swapchain.window_extent = window_extent;
        Swapchain::init(&mut swapchain, None, device);

        return swapchain;
    }

    pub fn renew(device: &Device, window_extent: vk::Extent2D, previous: &Swapchain) -> Swapchain {
        let mut swapchain = Swapchain::default();
        swapchain.window_extent = window_extent;
        Swapchain::init(
            &mut swapchain,
            Some(previous.swapchain.as_ref().unwrap().swapchain),
            device,
        );

        return swapchain;
    }

    fn init(self: &mut Swapchain, old_swapchain: Option<vk::SwapchainKHR>, device: &Device) {
        Swapchain::create_swapchain(self, device, old_swapchain);
        Swapchain::create_image_views(self, device);
        Swapchain::create_renderpass(self, device);
        Swapchain::create_depth_resources(self, device);
        Swapchain::create_framebuffers(self, device);
        Swapchain::create_sync_objects(self, device);
    }

    pub fn default() -> Swapchain {
        return Swapchain {
            swapchain_image_format: None,
            swapchain_depth_format: None,
            swapchain_extent: None,
            swapchain_framebuffers: Vec::new(),
            renderpass: None,
            depth_images: Vec::new(),
            swapchain_images: Vec::new(),
            window_extent: vk::Extent2D::default(),
            swapchain: None,
            image_available_semaphores: Vec::new(),
            render_finished_semaphores: Vec::new(),
            in_flight_fences: Vec::new(),
            images_in_flight: Vec::new(),
            current_frame: 0,
        };
    }

    pub fn get_framebuffer(&self, index: usize) -> vk::Framebuffer {
        return self.swapchain_framebuffers[index].get_framebuffer();
    }

    pub fn get_renderpass(&self) -> vk::RenderPass {
        return self.renderpass.unwrap();
    }

    pub fn get_image_view(&self, index: usize) -> vk::ImageView {
        return self.swapchain_images[index].get_image_view();
    }

    pub fn image_count(&self) -> usize {
        return self.swapchain_images.len();
    }

    pub fn get_swapchain_image_format(&self) -> vk::Format {
        return self.swapchain_image_format.unwrap();
    }

    pub fn get_swapchain_extent(&self) -> vk::Extent2D {
        return self.swapchain_extent.unwrap();
    }

    pub fn extent_aspect_ratio(&self) -> f32 {
        return self.swapchain_extent.unwrap().width as f32
            / self.swapchain_extent.unwrap().height as f32;
    }

    pub fn find_depth_format(&self, device: &Device) -> vk::Format {
        return device.find_support_format(
            &[
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        );
    }

    pub fn acquire_next_image(&self, device: &Device) -> Result<(u32, bool), vk::Result> {
        unsafe {
            device
                .device()
                .wait_for_fences(&[self.in_flight_fences[self.current_frame]], true, u64::MAX)
                .expect("Failed to wait for fences!");

            let result = self
                .swapchain
                .as_ref()
                .unwrap()
                .swapchain_loader
                .acquire_next_image(
                    self.swapchain.as_ref().unwrap().swapchain,
                    std::u64::MAX,
                    self.image_available_semaphores[self.current_frame],
                    vk::Fence::null(),
                );

            return result;
        }
    }

    pub fn compare_swap_formats(&self, swapchain: &Swapchain) -> bool {
        return swapchain.swapchain_depth_format.unwrap() == self.swapchain_depth_format.unwrap()
            && swapchain.swapchain_image_format.unwrap() == self.swapchain_image_format.unwrap();
    }

    pub fn submit_command_buffers(
        &mut self,
        device: &Device,
        buffer: vk::CommandBuffer,
        image_index: u32,
    ) -> Result<bool, vk::Result> {
        if self.images_in_flight[image_index as usize] != vk::Fence::null() {
            unsafe {
                device
                    .device()
                    .wait_for_fences(
                        &[self.images_in_flight[image_index as usize]],
                        true,
                        u64::MAX,
                    )
                    .expect("Failed to wait for fences!");
            }
        }

        self.images_in_flight[image_index as usize] = self.in_flight_fences[self.current_frame];

        let wait_semaphores: [vk::Semaphore; 1] =
            [self.image_available_semaphores[self.current_frame]];

        let wait_stages: [vk::PipelineStageFlags; 1] =
            [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let signal_semaphores: [vk::Semaphore; 1] =
            [self.render_finished_semaphores[self.current_frame as usize]];

        let submit_info: vk::SubmitInfo = vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &buffer,
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
        };

        unsafe {
            device
                .device()
                .reset_fences(&[self.in_flight_fences[self.current_frame]])
                .expect("Failed to reset fences!");
            device
                .device()
                .queue_submit(
                    device.graphics_queue(),
                    &[submit_info],
                    self.in_flight_fences[self.current_frame],
                )
                .expect("Failed to submit draw command buffer!");
        }

        let swapchains: [vk::SwapchainKHR; 1] = [self.swapchain.as_ref().unwrap().swapchain];

        let present_info: vk::PresentInfoKHR = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_count: signal_semaphores.len() as u32,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: swapchains.len() as u32,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: std::ptr::null_mut(),
        };

        let result = unsafe {
            self.swapchain
                .as_ref()
                .unwrap()
                .swapchain_loader
                .queue_present(device.present_queue(), &present_info)
        };

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;

        return result;
    }

    fn create_swapchain(
        self: &mut Swapchain,
        device: &Device,
        old_swapchain: Option<vk::SwapchainKHR>,
    ) {
        let swapchain_support: SwapChainSupportDetails = device.get_swapchain_support();

        let surface_format: vk::SurfaceFormatKHR =
            self.choose_swap_surface_format(&swapchain_support.surface_formats);
        let present_mode: vk::PresentModeKHR = self
            .choose_swap_present_mode(&swapchain_support.present_modes)
            .unwrap();
        let extent: vk::Extent2D = self.choose_swap_extent(&swapchain_support.surface_capabilities);

        let image_count = swapchain_support.surface_capabilities.min_image_count + 1;
        let image_count = if swapchain_support.surface_capabilities.max_image_count > 0 {
            image_count.min(swapchain_support.surface_capabilities.max_image_count)
        } else {
            image_count
        };

        let indices: QueueFamily = device.find_physical_queue_families();

        let (image_sharing, queue_family_index_count, queue_family_indices) = {
            if indices.graphics_family != indices.present_family {
                (
                    vk::SharingMode::CONCURRENT,
                    2,
                    vec![indices.graphics_family, indices.present_family],
                )
            } else {
                (vk::SharingMode::EXCLUSIVE, 0, vec![])
            }
        };

        let old_swapchain = if old_swapchain.is_none() {
            vk::SwapchainKHR::null()
        } else {
            old_swapchain.unwrap()
        };

        let create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: device.surface(),
            min_image_count: image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: image_sharing,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count: queue_family_index_count,
            pre_transform: swapchain_support.surface_capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: present_mode,
            clipped: vk::TRUE,
            old_swapchain: old_swapchain,
            image_array_layers: 1,
        };

        unsafe {
            let swapchain_loader = ash::extensions::khr::Swapchain::new(
                &device.instance.as_ref().unwrap(),
                &device.device(),
            );

            let _swapchain = swapchain_loader
                .create_swapchain(&create_info, None)
                .expect("Failed to create swapchain!");

            let vulkan_images = swapchain_loader.get_swapchain_images(_swapchain).unwrap();

            for vulkan_image in vulkan_images {
                let image = Image::new_swapchain(surface_format.format, extent, vulkan_image);
                self.swapchain_images.push(image);
            }

            self.swapchain_image_format = Some(surface_format.format);
            self.swapchain_extent = Some(extent);

            self.swapchain = Some(SwapchainKHR {
                swapchain: _swapchain,
                swapchain_loader: swapchain_loader,
            });
        }
    }

    fn create_image_views(self: &mut Swapchain, device: &Device) {
        for image in self.swapchain_images.iter_mut() {
            image.new_image_view(device, vk::ImageAspectFlags::COLOR);
        }
    }

    fn create_renderpass(self: &mut Swapchain, device: &Device) {
        let color_attachment = vk::AttachmentDescription {
            format: self.swapchain_image_format.unwrap(),
            flags: vk::AttachmentDescriptionFlags::empty(),
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        };

        let color_attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let depth_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: self.find_depth_format(device),
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::DONT_CARE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let subpasses = [vk::SubpassDescription {
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            p_depth_stencil_attachment: &depth_attachment_ref,
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            p_resolve_attachments: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        }];

        let dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        }];

        let attachments: [vk::AttachmentDescription; 2] = [color_attachment, depth_attachment];

        let create_info: vk::RenderPassCreateInfo = vk::RenderPassCreateInfo {
            flags: vk::RenderPassCreateFlags::empty(),
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: subpasses.len() as u32,
            p_subpasses: subpasses.as_ptr(),
            dependency_count: dependencies.len() as u32,
            p_dependencies: dependencies.as_ptr(),
            p_next: std::ptr::null(),
        };

        unsafe {
            self.renderpass = Some(
                device
                    .device()
                    .create_render_pass(&create_info, None)
                    .expect("Failed to create render pass!"),
            );
        }
    }

    fn create_framebuffers(self: &mut Swapchain, device: &Device) {
        let image_count = self.image_count();

        for i in 0..self.image_count() {
            let attachments: [vk::ImageView; 2] = [
                self.swapchain_images[i].get_image_view(),
                self.depth_images[i].get_image_view(),
            ];

            let _swapchain_extent: vk::Extent2D = self.get_swapchain_extent();

            let framebuffer = Framebuffer::new(device,attachments,self.renderpass.unwrap(),_swapchain_extent.width,_swapchain_extent.height);

            self.swapchain_framebuffers.push(framebuffer);
        }
    }

    fn create_sync_objects(self: &mut Swapchain, device: &Device) {
        self.image_available_semaphores
            .resize(MAX_FRAMES_IN_FLIGHT, vk::Semaphore::null());
        self.render_finished_semaphores
            .resize(MAX_FRAMES_IN_FLIGHT, vk::Semaphore::null());
        self.in_flight_fences
            .resize(MAX_FRAMES_IN_FLIGHT, vk::Fence::null());
        self.images_in_flight
            .resize(MAX_FRAMES_IN_FLIGHT, vk::Fence::null());

        let semaphore_info: vk::SemaphoreCreateInfo = vk::SemaphoreCreateInfo::default();

        let fence_info: vk::FenceCreateInfo = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
        };

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                self.image_available_semaphores[i] = device
                    .device()
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create the first sync object semaphore!");
                self.render_finished_semaphores[i] = device
                    .device()
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create the second sync object semaphore!");
                self.in_flight_fences[i] = device
                    .device()
                    .create_fence(&fence_info, None)
                    .expect("Failed to create the sync object fence!");
            }
        }
    }

    fn create_depth_resources(self: &mut Swapchain, device: &Device) {
        let depth_format: vk::Format = self.find_depth_format(device);
        self.swapchain_depth_format = Some(depth_format);

        let image_count = self.image_count();

        for i in 0..self.image_count() {
            let mut image = Image::new_2d(
                device,
                depth_format,
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                self.swapchain_extent.unwrap().width,
                self.swapchain_extent.unwrap().height,
            );

            image.new_image_view(device, vk::ImageAspectFlags::DEPTH);

            self.depth_images.push(image);
        }
    }

    fn choose_swap_surface_format(
        &self,
        available_formats: &Vec<vk::SurfaceFormatKHR>,
    ) -> vk::SurfaceFormatKHR {
        for available_format in available_formats {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return available_format.clone();
            }
        }

        return available_formats.first().unwrap().clone();
    }

    fn choose_swap_present_mode(
        &self,
        available_present_modes: &Vec<vk::PresentModeKHR>,
    ) -> Option<vk::PresentModeKHR> {
        for present_mode in available_present_modes {
            if *present_mode == vk::PresentModeKHR::MAILBOX {
                return Some(*present_mode);
            }
        }

        return Some(vk::PresentModeKHR::FIFO);
    }

    fn choose_swap_extent(&self, surface_capabilites: &vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if surface_capabilites.current_extent.width != u32::MAX {
            return surface_capabilites.current_extent;
        } else {
            if surface_capabilites.current_extent.width != u32::max_value() {
                surface_capabilites.current_extent
            } else {
                use num::clamp;

                let window_size = self.window_extent;
                println!(
                    "\t\tInner Window Size: ({}, {})",
                    window_size.width, window_size.height
                );

                vk::Extent2D {
                    width: clamp(
                        window_size.width as u32,
                        surface_capabilites.min_image_extent.width,
                        surface_capabilites.max_image_extent.width,
                    ),
                    height: clamp(
                        window_size.height as u32,
                        surface_capabilites.min_image_extent.height,
                        surface_capabilites.max_image_extent.height,
                    ),
                }
            }
        }
    }

    pub unsafe fn cleanup(&mut self, device: &Device) {
        self.swapchain_images
            .iter_mut()
            .for_each(|image| image.clean_view(device));

        self.swapchain
            .as_ref()
            .unwrap()
            .swapchain_loader
            .destroy_swapchain(self.swapchain.as_ref().unwrap().swapchain, None);

        self.depth_images
            .iter_mut()
            .for_each(|image| image.clean_view(device));

        self.depth_images
            .iter_mut()
            .for_each(|image| image.clean_image(device));

        self.depth_images
            .iter_mut()
            .for_each(|image| image.clean_memory(device));

        self.swapchain_framebuffers
            .iter_mut()
            .for_each(|framebuffer| framebuffer.clean_framebuffer(device));

        device
            .device()
            .destroy_render_pass(self.renderpass.unwrap(), None);

        self.render_finished_semaphores
            .iter()
            .for_each(|s| device.device().destroy_semaphore(*s, None));

        self.image_available_semaphores
            .iter()
            .for_each(|s| device.device().destroy_semaphore(*s, None));

        self.in_flight_fences
            .iter()
            .for_each(|f| device.device().destroy_fence(*f, None));
    }
}

/*#[cfg(test)]
mod tests {

    use crate::engine::device::Device;
    use crate::engine::swapchain::Swapchain;
    use crate::engine::window::Window;

    #[test]
    fn create_image_views_test() {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Visible(true));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let window = Window::new(&mut glfw, "Revier:DEV BUILD #1", 640, 480);
        let device = Device::new(&window, &glfw);
        let mut swapchain = Swapchain::default();

        Swapchain::create_image_views(&mut swapchain, &device);

        assert_eq!(swapchain.swapchain_image_views.len() > 0, true);
    }

    #[test]
    fn create_render_pass_test() {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Visible(true));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let window = Window::new(&mut glfw, "Revier:DEV BUILD #1", 640, 480);
        let device = Device::new(&window, &glfw);
        let mut swapchain = Swapchain::default();

        Swapchain::create_renderpass(&mut swapchain, &device);

        assert_eq!(swapchain.renderpass.is_some(), true)
    }
}*/
