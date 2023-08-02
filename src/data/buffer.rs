use crate::engine::device::Device;
use ash::vk;
use std::{
    any::{Any, TypeId},
    ffi::c_void,
};

pub struct Buffer {
    pub mapped: Option<*mut c_void>,
    pub memory: Option<vk::DeviceMemory>,
    pub buffer: Option<vk::Buffer>,
    pub buffer_size: Option<vk::DeviceSize>,
    pub usage_flags: Option<vk::BufferUsageFlags>,
    pub memory_property_flags: Option<vk::MemoryPropertyFlags>,
    pub instance_size: Option<vk::DeviceSize>,
    pub alignment_size: Option<vk::DeviceSize>,
    pub instance_count: u32,
}

impl Buffer {
    pub fn new(
        device: &Device,
        instance_size: vk::DeviceSize,
        instance_count: u32,
        usage_flags: vk::BufferUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        min_offset_alignment: Option<vk::DeviceSize>,
    ) -> Self {
        let new_min_offset_alignment: vk::DeviceSize;

        if min_offset_alignment.is_none() {
            new_min_offset_alignment = 1;
        } else {
            new_min_offset_alignment = min_offset_alignment.unwrap();
        }

        let alignment_size = Buffer::get_alignment(instance_size, new_min_offset_alignment);

        let buffer_size = alignment_size * instance_count as u64;

        let (buffer,memory) = device.create_buffer(buffer_size, usage_flags, memory_property_flags);

        return Self {
            mapped: None,
            memory: Some(memory),
            buffer: Some(buffer),
            buffer_size: Some(buffer_size),
            usage_flags: Some(usage_flags),
            memory_property_flags: Some(memory_property_flags),
            instance_count: instance_count,
            instance_size: Some(instance_size),
            alignment_size: Some(alignment_size),
        };
    }

    fn get_alignment(
        instance_size: vk::DeviceSize,
        min_offset_alignment: vk::DeviceSize,
    ) -> vk::DeviceSize {
        if min_offset_alignment > 0 {
            return (instance_size + min_offset_alignment - 1) & !(min_offset_alignment - 1);
        }

        return instance_size;
    }

    pub fn map(
        &mut self,
        device: &Device,
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
    ) {
        let new_size: vk::DeviceSize;
        let new_offset: vk::DeviceSize;

        if size.is_none() {
            new_size = vk::WHOLE_SIZE;
        } else {
            new_size = size.unwrap();
        }

        if offset.is_none() {
            new_offset = 0;
        } else {
            new_offset = offset.unwrap();
        }

        unsafe {
            self.mapped = Some(
                device
                    .device()
                    .map_memory(
                        self.memory.unwrap(),
                        new_offset,
                        new_size,
                        vk::MemoryMapFlags::empty(),
                    )
                    .expect("Failed to map memory on the buffer!"),
            );
        }
    }

    pub fn flush(
        &self,
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
        device: &Device,
    ) {
        let new_size: vk::DeviceSize;
        let new_offset: vk::DeviceSize;

        if size.is_none() {
            new_size = vk::WHOLE_SIZE;
        } else {
            new_size = size.unwrap();
        }

        if offset.is_none() {
            new_offset = 0;
        } else {
            new_offset = offset.unwrap();
        }

        let mapped_range: [vk::MappedMemoryRange; 1] = [vk::MappedMemoryRange {
            s_type: vk::StructureType::MAPPED_MEMORY_RANGE,
            p_next: std::ptr::null(),
            memory: self.memory.unwrap(),
            size: new_size,
            offset: new_offset,
        }];

        unsafe {
            return device
                .device()
                .flush_mapped_memory_ranges(&mapped_range)
                .expect("Failed to flush memory from the buffer!");
        }
    }

    pub fn write_to_buffer(&self,data:*mut c_void){

    }

    pub fn get_buffer(&self) -> vk::Buffer {
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
