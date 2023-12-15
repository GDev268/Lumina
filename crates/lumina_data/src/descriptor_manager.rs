use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
    u128::MAX,
};

use ash::vk;
use lumina_core::{device::Device, image::Image, swapchain::MAX_FRAMES_IN_FLIGHT};

use crate::{
    buffer::Buffer,
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, LayoutConfig},
};

pub struct DescriptorInformation {
    type_id: Option<std::any::TypeId>,
    buffers: Vec<Buffer>,
    images: Vec<Image>,
    binding: u32,
    is_uniform: bool,
    was_created: bool,
}

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
                    was_created: false,
                },
            );

            println!("BEFORE: {:?}", label);

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
                    64,
                    64,
                );

                image.new_image_view(&self.device, vk::ImageAspectFlags::COLOR);

                values.images.push(image);
            }

            values.was_created = true;
        }
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

    pub fn change_buffer_value<T: Any>(&mut self, label: String, cur_frame: u32, value: &[T]) {
        let cur_struct = self
            .descriptor_table
            .get_mut(&label)
            .expect("Failed to get the value!");

        if std::mem::size_of_val(value) as u64
            == cur_struct.buffers[cur_frame as usize].get_buffer_size()
        {
            cur_struct.buffers[cur_frame as usize].write_to_buffer(value, None, None);
            cur_struct.buffers[cur_frame as usize].flush(None, None)
        }
    }

    pub fn get_descriptor_set(&mut self, cur_frame: u32) -> vk::DescriptorSet {
        return self.descriptor_sets[cur_frame as usize];
    }

    pub fn get_descriptor_layout(&self) -> &DescriptorSetLayout {
        return &self.descriptor_set_layout;
    }
}
