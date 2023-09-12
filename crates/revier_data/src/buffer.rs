use revier_core::device::Device;

use ash::vk;
use std::{ffi::c_void};

pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub mapped: *mut c_void,
    pub buffer_size: u64,
}

impl Buffer {
    pub fn new(
        device: &Device,
        instance_size: u64,
        instance_count: u64,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> Self {
        let buffer_size = instance_size * instance_count;

        let (vertex_buffer, vertex_buffer_memory) = device.create_buffer(
            buffer_size,
            usage,
            properties
        );

        Self {
            buffer: vertex_buffer,
            memory: vertex_buffer_memory,
            mapped: std::ptr::null_mut(),
            buffer_size: buffer_size,
        }
    }

    fn get_alignment(
        instance_size: vk::DeviceSize,
        min_offset_alignment: vk::DeviceSize,
    ) -> vk::DeviceSize {
        vk::DeviceSize::default();
        if min_offset_alignment > 0 {
            return (instance_size + min_offset_alignment - 1) & !(min_offset_alignment - 1);
        }

        return instance_size;
    }

    pub fn map_to_buffer<T>(&mut self, device: &Device,data: &[T],_offset: vk::DeviceSize) {
        unsafe{
        self.mapped = device
            .device()
            .map_memory(
                self.memory,
                0,
                self.buffer_size,
                vk::MemoryMapFlags::empty(),
            )
            .expect("Failed to map memory on the buffer!");

        std::ptr::copy_nonoverlapping(data.as_ptr(), self.mapped as *mut T, data.len());
        }

    }

    pub fn map(size:Option<vk::DeviceSize>,offset: Option<vk::DeviceSize>){
        let new_size = if size.is_none(){
            vk::WHOLE_SIZE
        }else{
            size.unwrap()
        };

        let new_offset = if offset.is_none(){
            0
        }else{
            offset.unwrap()
        };
    }

    pub fn write_to_buffer<T>(&mut self,data: &[T]){
        unsafe{
            std::ptr::copy_nonoverlapping(data.as_ptr(), self.mapped as *mut T, data.len());
        }
    }



    pub fn unmap(&self,device: &Device){
        unsafe {
            device.device().unmap_memory(self.memory);
        }
    }


    pub fn flush(&self, size: vk::DeviceSize, offset: vk::DeviceSize, device: &Device) {
        let mapped_range: [vk::MappedMemoryRange; 1] = [vk::MappedMemoryRange {
            s_type: vk::StructureType::MAPPED_MEMORY_RANGE,
            p_next: std::ptr::null(),
            memory: self.memory,
            size: size,
            offset: offset,
        }];

        unsafe {
            return device
                .device()
                .flush_mapped_memory_ranges(&mapped_range)
                .expect("Failed to flush memory from the buffer!");
        }
    }

    pub fn get_buffer(&self) -> vk::Buffer {
        return self.buffer;
    }

    pub fn descriptor_info(
        &self,
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
    ) -> vk::DescriptorBufferInfo {
        let mut new_size: vk::DeviceSize = 0;
        let mut new_offset: vk::DeviceSize = 0;

        if size.is_none() {
            new_size = vk::WHOLE_SIZE;
        }

        if offset.is_none() {
            new_offset = 0;
        }

        return vk::DescriptorBufferInfo {
            buffer: self.buffer,
            offset: new_offset,
            range: new_size,
        };
    }
}
