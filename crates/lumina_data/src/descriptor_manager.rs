use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
    sync::Arc,
    u128::MAX,
};

use ash::vk;
use image::DynamicImage;
use lumina_atlas::atlas::Atlas;
use lumina_core::{
    device::{self, Device},
    image::Image,
    swapchain::MAX_FRAMES_IN_FLIGHT,
    texture::Texture,
};

use crate::{
    buffer::{self, Buffer},
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, LayoutConfig},
};

#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum CurValue {
    UNIFORM_BUFFER = 0,
    COLOR_IMAGE = 1,
    DEPTH_IMAGE = 2,
    CUBEMAP_COLOR_IMAGE = 3,
    CUBEMAP_DEPTH_IMAGE = 4,
}

pub struct DescriptorInformation {
    type_id: Option<std::any::TypeId>,
    pub buffers: Vec<Buffer>,
    images: Vec<Image>,
    binding: u32,
    buffer_sizes: (u64, u64),
    image_size: (u32, u32),
    pub value: CurValue,
}

pub struct DescriptorManager {
    device: Arc<Device>,
    pub descriptor_table: HashMap<String, DescriptorInformation>,
    pub layout_config: LayoutConfig,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub descriptor_sets: Vec<vk::DescriptorSet>,
    pub descriptor_pool: DescriptorPool,
    descriptor_done: bool,
    pub descriptor_positions: Vec<(u32, bool, String)>,
}

impl DescriptorManager {
    pub fn new(device: Arc<Device>, descriptor_pool: DescriptorPool) -> Self {
        Self {
            device: device,
            descriptor_table: HashMap::new(),
            layout_config: LayoutConfig::new(),
            descriptor_set_layout: DescriptorSetLayout::default(),
            descriptor_sets: Vec::new(),
            descriptor_pool,
            descriptor_done: false,
            descriptor_positions: Vec::new(),
        }
    }

    pub fn add_new_descriptor(
        &mut self,
        label: String,
        binding: u32,
        value: CurValue,
        buffer_size: u64,
        value_capacity: Option<u32>,
    ) {
        let count = if value_capacity.is_some() {
            value_capacity.unwrap()
        } else {
            1
        };

        let mut is_uniform = false;

        if !self.descriptor_table.contains_key(&label) {
            if value == CurValue::UNIFORM_BUFFER {
                self.layout_config.add_binding(
                    binding,
                    vk::DescriptorType::UNIFORM_BUFFER,
                    vk::ShaderStageFlags::ALL_GRAPHICS,
                    count,
                );
                is_uniform = true;
            } else {
                self.layout_config.add_binding(
                    binding,
                    vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    vk::ShaderStageFlags::ALL_GRAPHICS,
                    1,
                );
                is_uniform = false;
            }

            self.descriptor_table.insert(
                label.clone(),
                DescriptorInformation {
                    type_id: None,
                    buffers: Vec::new(),
                    images: Vec::new(),
                    binding: binding.clone(),
                    buffer_sizes: (buffer_size, 1),
                    image_size: (64, 64),
                    value,
                },
            );

            if self.descriptor_positions.is_empty()
                || binding as usize >= self.descriptor_positions.len() - 1
            {
                self.descriptor_positions
                    .push((binding, is_uniform, label.clone()));
            } else if (binding as usize) < self.descriptor_positions.len() - 1 {
                self.descriptor_positions
                    .insert(binding.clone() as usize, (binding, is_uniform, label));
            }

            /*Quick sorting algorithm to make the program more safe to run on startup because of data inconsistencies  */
            for i in 0..self.descriptor_positions.len() {
                for j in 0..self.descriptor_positions.len() {
                    if self.descriptor_positions[i].0 < self.descriptor_positions[j].0 {
                        let temp = (
                            self.descriptor_positions[i].0,
                            self.descriptor_positions[i].1,
                            self.descriptor_positions[i].2.clone(),
                        );
                        self.descriptor_positions[i] = (
                            self.descriptor_positions[j].0,
                            self.descriptor_positions[j].1,
                            self.descriptor_positions[j].2.clone(),
                        );
                        self.descriptor_positions[j] = temp;
                    }
                }
            }
        }
    }

    pub fn build_descriptor(&mut self, label: &str, count: u64) {
        let values = self.descriptor_table.get_mut(&label.to_string()).unwrap();

        values.buffer_sizes.1 = count;
        for i in 0..MAX_FRAMES_IN_FLIGHT {
            match values.value {
                CurValue::UNIFORM_BUFFER => {
                    let mut buffer = Buffer::new(
                        Arc::clone(&self.device),
                        values.buffer_sizes.0,
                        count,
                        vk::BufferUsageFlags::UNIFORM_BUFFER,
                        vk::MemoryPropertyFlags::HOST_VISIBLE,
                    );

                    buffer.map(None, None);
                    values.buffers.push(buffer);
                }
                CurValue::COLOR_IMAGE => {
                    let buffer_size = values.image_size.0 * values.image_size.1 * 4;

                    let mut buffer = Buffer::new(
                        Arc::clone(&self.device),
                        buffer_size as u64,
                        1,
                        vk::BufferUsageFlags::TRANSFER_SRC,
                        vk::MemoryPropertyFlags::HOST_VISIBLE,
                    );

                    buffer.map(None, None);

                    values.buffers.push(buffer);

                    let mut image = Image::new_2d(
                        &self.device,
                        vk::Format::R8G8B8A8_SRGB,
                        vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
                        vk::MemoryPropertyFlags::DEVICE_LOCAL,
                        values.image_size.0,
                        values.image_size.1,
                    );

                    image.new_image_view(&self.device, vk::ImageAspectFlags::COLOR);

                    values.images.push(image);
                }
                CurValue::DEPTH_IMAGE => {
                    let buffer_size = values.image_size.0 * values.image_size.1;

                    let mut buffer = Buffer::new(
                        Arc::clone(&self.device),
                        buffer_size as u64,
                        1,
                        vk::BufferUsageFlags::TRANSFER_SRC,
                        vk::MemoryPropertyFlags::HOST_VISIBLE,
                    );

                    buffer.map(None, None);

                    values.buffers.push(buffer);

                    let mut image = Image::new_2d(
                        &self.device,
                        vk::Format::D32_SFLOAT,
                        vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
                        vk::MemoryPropertyFlags::DEVICE_LOCAL,
                        values.image_size.0,
                        values.image_size.1,
                    );

                    image.new_image_view(&self.device, vk::ImageAspectFlags::DEPTH);

                    values.images.push(image);
                }
                CurValue::CUBEMAP_COLOR_IMAGE => {
                    let buffer_size = values.image_size.0 * values.image_size.1 * 4;

                    let mut buffer = Buffer::new(
                        Arc::clone(&self.device),
                        buffer_size as u64,
                        6,
                        vk::BufferUsageFlags::TRANSFER_SRC,
                        vk::MemoryPropertyFlags::HOST_VISIBLE,
                    );

                    buffer.map(None, None);

                    values.buffers.push(buffer);

                    let mut image = Image::new_3d(
                        &self.device,
                        vk::Format::R8G8B8A8_SRGB,
                        vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
                        vk::MemoryPropertyFlags::DEVICE_LOCAL,
                        values.image_size.0,
                        values.image_size.1,
                    );

                    image.new_3d_image_view(&self.device, vk::ImageAspectFlags::COLOR);

                    values.images.push(image);
                }
                CurValue::CUBEMAP_DEPTH_IMAGE => unimplemented!(),
            }
        }
    }

    pub fn change_buffer_count(&mut self, label: &str, value_count: u64) {
        let binding = self.descriptor_table.get(label).unwrap().binding;

        if self.descriptor_table.get(label).unwrap().value == CurValue::UNIFORM_BUFFER {
            self.layout_config
                .change_binding_count(binding, value_count as u32);

            self.descriptor_table
                .get_mut(label)
                .unwrap()
                .buffers
                .clear();

            self.build_descriptor(label, value_count);
        }
    }

    pub fn change_image_size(&mut self, label: &str, width: u32, height: u32) {
        let binding = self.descriptor_table.get(label).unwrap().binding;

        self.descriptor_table.get_mut(label).unwrap().image_size = (width, height);

        if self.descriptor_table.get(label).unwrap().value == CurValue::COLOR_IMAGE
            || self.descriptor_table.get(label).unwrap().value == CurValue::DEPTH_IMAGE
            || self.descriptor_table.get(label).unwrap().value == CurValue::CUBEMAP_COLOR_IMAGE
        {
            for image in &mut self.descriptor_table.get_mut(label).unwrap().images {
                image.clean_image(&self.device);
                image.clean_view(&self.device);
                image.clean_memory(&self.device);
            }

            for buffer in &mut self.descriptor_table.get_mut(label).unwrap().buffers {
                buffer.flush(None, None);
                buffer.unmap();
            }

            self.descriptor_table.get_mut(label).unwrap().images.clear();
            self.descriptor_table
                .get_mut(label)
                .unwrap()
                .buffers
                .clear();
        }

        self.build_descriptor(label, 1);

    }

    pub fn preload_we(&mut self) {
        self.descriptor_set_layout = self.layout_config.build(&self.device);
        let descriptor_set = self.descriptor_pool.allocate_descriptor(
            &self.device,
            self.descriptor_set_layout.get_descriptor_set_layout(),
        );

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            let mut writers = Vec::new();

            let mut buffers: HashMap<u32, vk::DescriptorBufferInfo> = HashMap::new();
            let mut images: HashMap<u32, vk::DescriptorImageInfo> = HashMap::new();

            for (binding, is_uniform, name) in &self.descriptor_positions {
                if *is_uniform {
                    buffers.insert(
                        *binding,
                        self.descriptor_table.get(name).unwrap().buffers[i]
                            .descriptor_info(None, None),
                    );
                } else {
                    images.insert(
                        *binding,
                        self.descriptor_table.get(name).unwrap().images[i].descriptor_info(),
                    );
                }
            }

            for values in self.descriptor_table.values() {
                if values.value == CurValue::UNIFORM_BUFFER {
                    let binding_description = self
                        .descriptor_set_layout
                        .bindings
                        .get(&values.binding)
                        .expect("Layout does not contain specified binding");

                    let write = vk::WriteDescriptorSet {
                        s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                        p_next: std::ptr::null(),
                        dst_set: descriptor_set,
                        dst_binding: values.binding,
                        dst_array_element: 0,
                        descriptor_count: 1,
                        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                        p_image_info: std::ptr::null(),
                        p_buffer_info: &buffers[&values.binding],
                        p_texel_buffer_view: std::ptr::null(),
                    };

                    writers.push(write);
                } else {
                    let binding_description = self
                        .descriptor_set_layout
                        .bindings
                        .get(&values.binding)
                        .expect("Layout does not contain specified binding");

                    let write = vk::WriteDescriptorSet {
                        s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                        p_next: std::ptr::null(),
                        dst_set: descriptor_set,
                        dst_binding: values.binding,
                        dst_array_element: 0,
                        descriptor_count: 1,
                        descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                        p_image_info: &images[&values.binding],
                        p_buffer_info: std::ptr::null(),
                        p_texel_buffer_view: std::ptr::null(),
                    };

                    writers.push(write);
                }
            }

            unsafe { self.device.device().update_descriptor_sets(&writers, &[]) };

            self.descriptor_sets.push(descriptor_set);
        }
    }

    pub fn update_we(&mut self) {
        self.descriptor_set_layout = self.layout_config.build(&self.device);

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            let mut writers = Vec::new();

            let mut buffers: HashMap<u32, vk::DescriptorBufferInfo> = HashMap::new();
            let mut images: HashMap<u32, vk::DescriptorImageInfo> = HashMap::new();

            for (binding, is_uniform, name) in &self.descriptor_positions {
                if *is_uniform {
                    buffers.insert(
                        *binding,
                        self.descriptor_table.get(name).unwrap().buffers[i]
                            .descriptor_info(None, None),
                    );
                } else {
                    images.insert(
                        *binding,
                        self.descriptor_table.get(name).unwrap().images[i].descriptor_info(),
                    );
                }
            }

            for values in self.descriptor_table.values() {
                if values.value == CurValue::UNIFORM_BUFFER {
                    let binding_description = self
                        .descriptor_set_layout
                        .bindings
                        .get(&values.binding)
                        .expect("Layout does not contain specified binding");

                    let write = vk::WriteDescriptorSet {
                        s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                        p_next: std::ptr::null(),
                        dst_set: self.descriptor_sets[i],
                        dst_binding: values.binding,
                        dst_array_element: 0,
                        descriptor_count: 1,
                        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                        p_image_info: std::ptr::null(),
                        p_buffer_info: &buffers[&values.binding],
                        p_texel_buffer_view: std::ptr::null(),
                    };

                    writers.push(write);
                } else {
                    let binding_description = self
                        .descriptor_set_layout
                        .bindings
                        .get(&values.binding)
                        .expect("Layout does not contain specified binding");

                    let write = vk::WriteDescriptorSet {
                        s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                        p_next: std::ptr::null(),
                        dst_set: self.descriptor_sets[i],
                        dst_binding: values.binding,
                        dst_array_element: 0,
                        descriptor_count: 1,
                        descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                        p_image_info: &images[&values.binding],
                        p_buffer_info: std::ptr::null(),
                        p_texel_buffer_view: std::ptr::null(),
                    };

                    writers.push(write);
                }
            }

            unsafe { self.device.device().update_descriptor_sets(&writers, &[]) };
        }
    }

    pub fn change_buffer_value<T: Any>(&mut self, label: &str, cur_frame: u32, values: &[T]) {
        if let Some(cur_struct) = self.descriptor_table.get_mut(&label.to_string()) {
            let value_size = std::mem::size_of::<T>();

            let max_size = (value_size * values.len()) as u64;

            if max_size == cur_struct.buffers[cur_frame as usize].get_buffer_size() {
                cur_struct.buffers[cur_frame as usize].write_to_buffer(values, None, None);
                cur_struct.buffers[cur_frame as usize].flush(None, None)
            }
        } else {
            eprintln!("ERROR: Value doesn't exist")
        }
    }

    pub fn change_image_value(&mut self, label: &str, value: &DynamicImage) {
        let cur_struct = self
            .descriptor_table
            .get_mut(&label.to_string())
            .expect("Failed to get the value!");

        if let Some(cur_struct) = self.descriptor_table.get_mut(&label.to_string()) {
            for i in 0..cur_struct.images.len() {
                cur_struct.buffers[i].write_to_buffer(&value.clone().into_bytes(), None, None);

                DescriptorManager::transition_image_layout(
                    Arc::clone(&self.device),
                    cur_struct.images[i].get_image(),
                    cur_struct.images[i].get_format(),
                    vk::ImageLayout::UNDEFINED,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                );

                let command_buffer: vk::CommandBuffer;

                let alloc_info = vk::CommandBufferAllocateInfo {
                    s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                    p_next: std::ptr::null(),
                    command_pool: self.device.get_command_pool(),
                    level: vk::CommandBufferLevel::PRIMARY,
                    command_buffer_count: 1,
                };

                command_buffer = unsafe {
                    self.device
                        .device()
                        .allocate_command_buffers(&alloc_info)
                        .unwrap()[0]
                };

                let begin_info = vk::CommandBufferBeginInfo {
                    s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                    p_next: std::ptr::null(),
                    flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                    ..Default::default()
                };

                unsafe {
                    self.device
                        .device()
                        .begin_command_buffer(command_buffer, &begin_info)
                        .expect("Failed to begin command buffer!");
                }

                let region = vk::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_row_length: 0,
                    buffer_image_height: 0,
                    image_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: 0,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                    image_extent: vk::Extent3D {
                        width: value.width(),
                        height: value.height(),
                        depth: 1,
                    },
                };

                unsafe {
                    self.device.device().cmd_copy_buffer_to_image(
                        command_buffer,
                        cur_struct.buffers[i].get_buffer(),
                        cur_struct.images[i].get_image(),
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &[region],
                    );
                }

                unsafe {
                    self.device
                        .device()
                        .end_command_buffer(command_buffer)
                        .expect("Failed to end command buffer!");
                    let submit_info = vk::SubmitInfo {
                        s_type: vk::StructureType::SUBMIT_INFO,
                        command_buffer_count: 1,
                        p_command_buffers: &command_buffer,
                        ..Default::default()
                    };
                    self.device
                        .device()
                        .queue_submit(
                            self.device.graphics_queue(),
                            &[submit_info],
                            vk::Fence::null(),
                        )
                        .expect("Failed to submit data");
                    self.device
                        .device()
                        .queue_wait_idle(self.device.graphics_queue())
                        .unwrap();
                    self.device
                        .device()
                        .free_command_buffers(self.device.get_command_pool(), &[command_buffer]);
                }

                DescriptorManager::transition_image_layout(
                    Arc::clone(&self.device),
                    cur_struct.images[i].get_image(),
                    cur_struct.images[i].get_format(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    vk::ImageLayout::GENERAL,
                );
            }
        } else {
            eprintln!("ERROR: Failed to get the value");
        }
    }

    pub fn change_cubemap_value(&mut self, label: &str, value: [&DynamicImage; 6]) {
        let cur_struct = self
            .descriptor_table
            .get_mut(&label.to_string())
            .expect("Failed to get the value!");

        let mut data = Vec::new();

        for image in value {
            image.flipv();
            data.extend(image.clone().into_bytes());
        }

        for i in 0..cur_struct.images.len() {
            cur_struct.buffers[i].write_to_buffer(&data, None, None);

            DescriptorManager::transition_image_layout(
                Arc::clone(&self.device),
                cur_struct.images[i].get_image(),
                cur_struct.images[i].get_format(),
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            );

            let command_buffer: vk::CommandBuffer;

            let alloc_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                command_pool: self.device.get_command_pool(),
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: 1,
            };

            command_buffer = unsafe {
                self.device
                    .device()
                    .allocate_command_buffers(&alloc_info)
                    .unwrap()[0]
            };

            let begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: std::ptr::null(),
                flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                ..Default::default()
            };

            unsafe {
                self.device
                    .device()
                    .begin_command_buffer(command_buffer, &begin_info)
                    .expect("Failed to begin command buffer!");
            }

            let region = vk::BufferImageCopy {
                buffer_offset: 0,
                buffer_row_length: 0,
                buffer_image_height: 0,
                image_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 6,
                },
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: vk::Extent3D {
                    width: value[0].width(),
                    height: value[0].height(),
                    depth: 1,
                },
            };

            unsafe {
                self.device.device().cmd_copy_buffer_to_image(
                    command_buffer,
                    cur_struct.buffers[i].get_buffer(),
                    cur_struct.images[i].get_image(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[region],
                );
            }

            unsafe {
                self.device
                    .device()
                    .end_command_buffer(command_buffer)
                    .expect("Failed to end command buffer!");
                let submit_info = vk::SubmitInfo {
                    s_type: vk::StructureType::SUBMIT_INFO,
                    command_buffer_count: 1,
                    p_command_buffers: &command_buffer,
                    ..Default::default()
                };
                self.device
                    .device()
                    .queue_submit(
                        self.device.graphics_queue(),
                        &[submit_info],
                        vk::Fence::null(),
                    )
                    .expect("Failed to submit data");
                self.device
                    .device()
                    .queue_wait_idle(self.device.graphics_queue())
                    .unwrap();
                self.device
                    .device()
                    .free_command_buffers(self.device.get_command_pool(), &[command_buffer]);
            }

            DescriptorManager::transition_image_layout(
                Arc::clone(&self.device),
                cur_struct.images[i].get_image(),
                cur_struct.images[i].get_format(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::ImageLayout::GENERAL,
            );
        }
    }

    pub fn change_image_value_vk(&mut self, label: &str, value: vk::Image) {
        let cur_struct = self
            .descriptor_table
            .get_mut(&label.to_string())
            .expect("Failed to get the value!");

        for i in 0..cur_struct.images.len() {
            DescriptorManager::transition_image_layout(
                Arc::clone(&self.device),
                cur_struct.images[i].get_image(),
                cur_struct.images[i].get_format(),
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            );

            let command_buffer: vk::CommandBuffer;

            let alloc_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                command_pool: self.device.get_command_pool(),
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: 1,
            };

            command_buffer = unsafe {
                self.device
                    .device()
                    .allocate_command_buffers(&alloc_info)
                    .unwrap()[0]
            };

            let begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: std::ptr::null(),
                flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                ..Default::default()
            };

            unsafe {
                self.device
                    .device()
                    .begin_command_buffer(command_buffer, &begin_info)
                    .expect("Failed to begin command buffer!");
            }

            let region = vk::ImageCopy {
                src_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                dst_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                extent: vk::Extent3D {
                    width: 1024,
                    height: 1024,
                    depth: 1,
                },
                src_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                dst_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                },
            };

            unsafe {
                self.device.device().cmd_copy_image(
                    command_buffer,
                    value,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    cur_struct.images[i].get_image(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[region],
                )
            };

            unsafe {
                self.device
                    .device()
                    .end_command_buffer(command_buffer)
                    .expect("Failed to end command buffer!");
                let submit_info = vk::SubmitInfo {
                    s_type: vk::StructureType::SUBMIT_INFO,
                    command_buffer_count: 1,
                    p_command_buffers: &command_buffer,
                    ..Default::default()
                };
                self.device
                    .device()
                    .queue_submit(
                        self.device.graphics_queue(),
                        &[submit_info],
                        vk::Fence::null(),
                    )
                    .expect("Failed to submit data");
                self.device
                    .device()
                    .queue_wait_idle(self.device.graphics_queue())
                    .unwrap();
                self.device
                    .device()
                    .free_command_buffers(self.device.get_command_pool(), &[command_buffer]);
            }

            DescriptorManager::transition_image_layout(
                Arc::clone(&self.device),
                cur_struct.images[i].get_image(),
                cur_struct.images[i].get_format(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::ImageLayout::GENERAL,
            );
        }
    }

    pub fn transition_image_layout(
        device: Arc<Device>,
        image: vk::Image,
        format: vk::Format,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
    ) {
        let mut memory_barrier = vk::ImageMemoryBarrier {
            s_type: vk::StructureType::IMAGE_MEMORY_BARRIER,
            p_next: std::ptr::null(),
            old_layout,
            new_layout,
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            image,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };

        let command_buffer: vk::CommandBuffer;

        let alloc_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            command_pool: device.get_command_pool(),
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: 1,
        };

        command_buffer = unsafe {
            device
                .device()
                .allocate_command_buffers(&alloc_info)
                .unwrap()[0]
        };

        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };

        unsafe {
            device
                .device()
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin command buffer!");
        }

        let src_stage: vk::PipelineStageFlags = vk::PipelineStageFlags::ALL_COMMANDS;
        let dst_stage: vk::PipelineStageFlags = vk::PipelineStageFlags::ALL_COMMANDS;

        unsafe {
            device.device().cmd_pipeline_barrier(
                command_buffer,
                src_stage,
                dst_stage,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[memory_barrier],
            )
        }

        unsafe {
            device
                .device()
                .end_command_buffer(command_buffer)
                .expect("Failed to end command buffer!");
            let submit_info = vk::SubmitInfo {
                s_type: vk::StructureType::SUBMIT_INFO,
                command_buffer_count: 1,
                p_command_buffers: &command_buffer,
                ..Default::default()
            };
            device
                .device()
                .queue_submit(device.graphics_queue(), &[submit_info], vk::Fence::null())
                .expect("Failed to submit data");
            device
                .device()
                .queue_wait_idle(device.graphics_queue())
                .unwrap();
            device
                .device()
                .free_command_buffers(device.get_command_pool(), &[command_buffer]);
        }
    }

    pub fn get_descriptor_set(&mut self, cur_frame: u32) -> vk::DescriptorSet {
        return self.descriptor_sets[cur_frame as usize];
    }

    pub fn get_descriptor_layout(&self) -> &DescriptorSetLayout {
        return &self.descriptor_set_layout;
    }

    pub fn drop_values(&mut self, device: &Device) {
        for (id, info) in self.descriptor_table.iter_mut() {
            if info.buffers.len() > 0 {
                for buffer in &info.buffers {
                    drop(buffer);
                }
            }

            if info.images.len() > 0 {
                for mut image in &mut info.images {
                    image.clean_memory(&device);
                    image.clean_image(&device);
                    image.clean_view(&device);
                }
            }
        }

        self.descriptor_pool.destroy(&device);
        self.descriptor_set_layout.destroy(&device);
    }
}
