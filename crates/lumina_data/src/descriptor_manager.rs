use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
    u128::MAX,
};

use ash::vk;
use lumina_core::{
    device::{self, Device},
    image::Image,
    swapchain::MAX_FRAMES_IN_FLIGHT,
    texture::Texture,
};

use crate::{
    buffer::Buffer,
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, LayoutConfig},
};

#[derive(Clone)]
pub struct DescriptorInformation {
    type_id: Option<std::any::TypeId>,
    buffers: Vec<Buffer>,
    images: Vec<Image>,
    binding: u32,
    is_uniform: bool,
}

#[derive(Clone)]
pub struct DescriptorManager {
    device: Rc<Device>,
    pub descriptor_table: HashMap<String, DescriptorInformation>,
    pub layout_config: LayoutConfig,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub descriptor_sets: Vec<vk::DescriptorSet>,
    pub descriptor_pool: DescriptorPool,
    descriptor_done: bool,
    pub descriptor_positions: Vec<(u32, bool, String)>,
}

impl DescriptorManager {
    pub fn new(device: Rc<Device>, descriptor_pool: DescriptorPool) -> Self {
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

    pub fn add_new_descriptor(&mut self, label: String, binding: u32, is_uniform: bool) {
        if !self.descriptor_table.contains_key(&label) {
            if is_uniform {
                self.layout_config.add_binding(
                    binding,
                    vk::DescriptorType::UNIFORM_BUFFER,
                    vk::ShaderStageFlags::ALL_GRAPHICS,
                    1,
                );
            } else {
                self.layout_config.add_binding(
                    binding,
                    vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    vk::ShaderStageFlags::ALL_GRAPHICS,
                    1,
                );
            }

            self.descriptor_table.insert(
                label.clone(),
                DescriptorInformation {
                    type_id: None,
                    buffers: Vec::new(),
                    images: Vec::new(),
                    binding: binding.clone(),
                    is_uniform,
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

    pub fn build_descriptor(&mut self, label: &str, buffer_size: u64) {
        let values = self.descriptor_table.get_mut(&label.to_string()).unwrap();

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            if values.is_uniform {
                let mut buffer = Buffer::new(
                    Rc::clone(&self.device),
                    buffer_size,
                    1,
                    vk::BufferUsageFlags::UNIFORM_BUFFER,
                    vk::MemoryPropertyFlags::HOST_VISIBLE,
                );

                buffer.map(None, None);
                values.buffers.push(buffer);
            } else {
                let mut image = Image::new_2d(
                    &self.device,
                    vk::Format::R8G8B8A8_SRGB,
                    vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
                    vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    800,
                    640,
                );

                image.new_image_view(&self.device, vk::ImageAspectFlags::COLOR);

                values.images.push(image);
            }
        }
    }

    pub fn preload_descriptors(&mut self) {
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
                if values.is_uniform {
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

    pub fn print_weege(&self) {
        println!("{:?}", self.descriptor_sets);
        println!("{:?}", self.descriptor_set_layout);
        println!("{:?}", self.descriptor_positions);
    }

    fn change_buffer(&mut self, label: String, value_size: usize, num_values: usize) {
        let cur_struct = self
            .descriptor_table
            .get_mut(&label)
            .expect("Failed to get the value!");

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            let mut buffer = Buffer::new(
                Rc::clone(&self.device),
                value_size as u64,
                num_values as u64,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE,
            );

            buffer.map(None, None);
            cur_struct.buffers[i] = buffer;
        }

        self.preload_descriptors()
    }

    pub fn change_buffer_value<T: Any>(&mut self, label: &str, cur_frame: u32, values: &[T]) {
        let cur_max_size = self
            .descriptor_table
            .get_mut(&label.to_string())
            .expect("Failed to get the value!")
            .buffers[cur_frame as usize]
            .get_buffer_size();

        let value_size = std::mem::size_of::<T>();

        let max_size = (value_size * values.len()) as u64;

        if max_size == cur_max_size {
            self.change_buffer(label.to_string().clone(), value_size, values.len())
        }

        let cur_struct = self
            .descriptor_table
            .get_mut(&label.to_string())
            .expect("Failed to get the value!");

        let value_size = std::mem::size_of::<T>();

        let max_size = (value_size * values.len()) as u64;

        if max_size == cur_struct.buffers[cur_frame as usize].get_buffer_size() {
            cur_struct.buffers[cur_frame as usize].write_to_buffer(values, None, None);
            cur_struct.buffers[cur_frame as usize].flush(None, None)
        }
    }

    pub fn change_image_value(&mut self, label: String, cur_frame: u32, value: Texture) {
        let cur_struct = self
            .descriptor_table
            .get_mut(&label)
            .expect("Failed to get the value!");

        DescriptorManager::transition_image_layout(
            Rc::clone(&self.device),
            cur_struct.images[cur_frame as usize].get_image(),
            cur_struct.images[cur_frame as usize].get_format(),
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        );

        let buffer_size =
            value.get_texture_info().0 * value.get_texture_info().1 * value.get_texture_info().2;

        let mut staging_buffer = Buffer::new(
            Rc::clone(&self.device),
            buffer_size as u64,
            1,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );

        staging_buffer.map(None, None);
        staging_buffer.write_to_buffer(&value.get_texture_data(), None, None);
        staging_buffer.flush(None, None);

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
                width: value.get_texture_info().0,
                height: value.get_texture_info().1,
                depth: 1,
            },
        };

        unsafe {
            self.device.device().cmd_copy_buffer_to_image(
                command_buffer,
                staging_buffer.get_buffer(),
                cur_struct.images[cur_frame as usize].get_image(),
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
            Rc::clone(&self.device),
            cur_struct.images[cur_frame as usize].get_image(),
            cur_struct.images[cur_frame as usize].get_format(),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::GENERAL,
        );
    }

    pub fn change_image_value_with_images(
        &mut self,
        label: String,
        cur_frame: u32,
        extent: vk::Extent2D,
        color_image: vk::Image,
        depth_image: vk::Image,
    ) {
        let cur_struct = self
            .descriptor_table
            .get_mut(&label)
            .expect("Failed to get the value!");

        DescriptorManager::transition_image_layout(
            Rc::clone(&self.device),
            cur_struct.images[cur_frame as usize].get_image(),
            cur_struct.images[cur_frame as usize].get_format(),
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

        // Allocate a new command buffer for each submission
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

        let color_image_copy = vk::ImageCopy {
            src_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            src_offset: vk::Offset3D::default(),
            dst_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            dst_offset: vk::Offset3D::default(),
            extent: vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            },
        };

        let depth_image_copy = vk::ImageCopy {
            src_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::DEPTH,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            src_offset: vk::Offset3D::default(),
            dst_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::DEPTH,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
            dst_offset: vk::Offset3D::default(),
            extent: vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            },
        };

        DescriptorManager::transition_image_layout(
            Rc::clone(&self.device),
            cur_struct.images[cur_frame as usize].get_image(),
            cur_struct.images[cur_frame as usize].get_format(),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::GENERAL,
        );

        unsafe {
            self.device.device().cmd_copy_image(
                command_buffer,
                color_image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                cur_struct.images[cur_frame as usize].get_image(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[color_image_copy],
            );

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
    }

    fn transition_image_layout(
        device: Rc<Device>,
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
}
