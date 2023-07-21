use ash::vk;
use std::any::{Any, TypeId};

pub struct Buffer {
    pub mapped: Option<Box<dyn Any>>,
    pub buffer: Option<vk::Buffer>,
    pub memory: Option<vk::DeviceMemory>,
    pub buffer_size: Option<vk::DeviceSize>,
    pub instance_count: u32,
    pub alignment_size: Option<vk::DeviceSize>,
    pub usage_flags: Option<vk::BufferUsageFlags>,
    pub memory_property_flags: Option<vk::MemoryPropertyFlags>
}

impl Buffer {
    pub fn new(
        instance_size: vk::DeviceSize,
        instance_count: u32,
        usage_flags: vk::BufferUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        min_offset_alignment: vk::DeviceSize,
    ) -> Self {

    }

    pub fn map(size: Option<vk::DeviceSize>, offset: Option<vk::DeviceSize>) {
        let mut new_size: vk::DeviceSize;
        let mut new_offset: vk::DeviceSize;

        if size.is_none() {
            new_size = vk::WHOLE_SIZE;
        }

        if offset.is_none() {
            new_offset = 0;
        }
    }

    pub fn flush(size: Option<vk::DeviceSize>, offset: Option<vk::DeviceSize>) {
        let mut new_size: vk::DeviceSize;
        let mut new_offset: vk::DeviceSize;

        if size.is_none() {
            new_size = vk::WHOLE_SIZE;
        }

        if offset.is_none() {
            new_offset = 0;
        }
    }

    pub fn get_buffer(&self) -> vk::Buffer{
        return self.buffer.unwrap();
    }

    pub fn descriptor_info(size: Option<vk::DeviceSize>, offset: Option<vk::DeviceSize>) {
        let mut new_size: vk::DeviceSize;
        let mut new_offset: vk::DeviceSize;

        if size.is_none() {
            new_size = vk::WHOLE_SIZE;
        }

        if offset.is_none() {
            new_offset = 0;
        }
    }
}
