use crate::device::Device;
use ash::vk;

pub struct Image {
    _image: vk::Image,
    format: vk::Format,
    extent: vk::Extent3D,
    memory: vk::DeviceMemory,
    _image_view: vk::ImageView,
    sampler: vk::Sampler,
    layout: vk::ImageLayout,
}

impl Image {
    pub fn new_2d(
        device: &Device,
        format: vk::Format,
        usage: vk::ImageUsageFlags,
        properties: vk::MemoryPropertyFlags,
        width: u32,
        height: u32,
    ) -> Self {
        let extent = vk::Extent3D {
            width: width,
            height: height,
            depth: 1,
        };

        let image_info = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::ImageCreateFlags::empty(),
            image_type: vk::ImageType::TYPE_2D,
            format: format,
            extent: vk::Extent3D {
                width: width,
                height: height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: std::ptr::null(),
            initial_layout: vk::ImageLayout::default(),
        };

        let sampler_info = vk::SamplerCreateInfo {
            mag_filter: vk::Filter::LINEAR,
            min_filter: vk::Filter::LINEAR,
            ..Default::default()
        };

        let sampler = unsafe {
            device
                .device()
                .create_sampler(&sampler_info, None)
                .expect("Failed to create image sampler")
        };

        unsafe {
            let (image, memory) = device.create_image_with_info(&image_info, properties);

            return Self {
                _image: image,
                memory,
                format,
                extent,
                _image_view: vk::ImageView::null(),
                sampler,
                layout: vk::ImageLayout::GENERAL,
            };
        }
    }

    pub fn new_swapchain(format: vk::Format, extent: vk::Extent2D, image: vk::Image) -> Self {
        let extent = vk::Extent3D::from(extent);
        Self {
            _image: image,
            format: format,
            extent: extent,
            memory: vk::DeviceMemory::null(),
            _image_view: vk::ImageView::null(),
            sampler: vk::Sampler::null(),
            layout: vk::ImageLayout::UNDEFINED,
        }
    }

    pub fn new_image_view(&mut self, device: &Device, aspect_mask: vk::ImageAspectFlags) {
        let view_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            image: self._image,
            p_next: std::ptr::null(),
            view_type: vk::ImageViewType::TYPE_2D,
            format: self.format,
            flags: vk::ImageViewCreateFlags::empty(),
            components: vk::ComponentMapping::default(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: aspect_mask,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };

        self._image_view = unsafe { device.device().create_image_view(&view_info, None).unwrap() };
    }

    pub fn get_image(&self) -> vk::Image {
        return self._image;
    }

    pub fn get_image_memory(&self) -> vk::DeviceMemory {
        return self.memory;
    }

    pub fn get_image_view(&self) -> vk::ImageView {
        return self._image_view;
    }

    pub fn descriptor_info(&self) -> vk::DescriptorImageInfo {
        return vk::DescriptorImageInfo {
            sampler: self.sampler,
            image_view: self._image_view,
            image_layout: vk::ImageLayout::GENERAL,
        };
    }

    pub fn clean_view(&mut self, device: &Device) {
        unsafe {
            device.device().destroy_image_view(self._image_view, None);
        }
    }

    pub fn clean_image(&mut self, device: &Device) {
        unsafe {
            device.device().destroy_image(self._image, None);
        }
    }

    pub fn clean_memory(&mut self, device: &Device) {
        unsafe {
            device.device().free_memory(self.memory, None);
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            _image: vk::Image::null(),
            format: vk::Format::default(),
            extent: vk::Extent3D::default(),
            memory: vk::DeviceMemory::null(),
            _image_view: vk::ImageView::null(),
            sampler: vk::Sampler::null(),
            layout: vk::ImageLayout::UNDEFINED,
        }
    }
}
