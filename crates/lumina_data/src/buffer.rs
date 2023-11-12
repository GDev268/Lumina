use lumina_core::device::Device;

use ash::vk;
use std::ffi::c_void;

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub mapped: *mut c_void,
    pub buffer_size: u64,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            buffer: vk::Buffer::null(), 
            memory: vk::DeviceMemory::null(), 
            mapped: std::ptr::null_mut(), 
            buffer_size: 0
        }
    }
}

impl Buffer {
    pub fn new(
        device: &Device,
        instance_size: u64,
        instance_count: u64,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
        min_offset_alignment: vk::DeviceSize,
    ) -> Self {
        let alignment_size = Buffer::get_alignment(instance_size, min_offset_alignment);
        let buffer_size = alignment_size * instance_count as u64;

        let (vertex_buffer, vertex_buffer_memory) =
            device.create_buffer(buffer_size, usage, properties);
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
        if min_offset_alignment > 0 {
            return (instance_size + min_offset_alignment - 1) & !(min_offset_alignment - 1);
        }

        instance_size
    }

    pub fn map(
        &mut self,
        device: &Device,
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
    ) {
        let new_size = size.unwrap_or(vk::WHOLE_SIZE);
        let new_offset = offset.unwrap_or(0);

        assert!(self.mapped.is_null(), "Memory is already mapped"); // Check if memory is already mapped

        unsafe {
            self.mapped = device
                .device()
                .map_memory(
                    self.memory,
                    new_offset,
                    new_size,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap();
        }
    }

    pub fn unmap(&self, device: &Device) {
        unsafe {
            device.device().unmap_memory(self.memory);
        }
    }

    pub fn write_to_buffer<T: Copy>(
        &mut self,
        data: &[T],
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
    ) {
        let new_size = if size.is_none() {
            self.buffer_size
        } else {
            size.unwrap()
        };

        let new_offset = if offset.is_none() { 0 } else { offset.unwrap() };

        unsafe {
            let mem_offset = self.mapped.add(new_offset as usize);

            let mut align =
                ash::util::Align::new(mem_offset, std::mem::align_of::<u32>() as u64, new_size);
            align.copy_from_slice(data);
        }
    }

    pub fn flush(
        &self,
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
        device: &Device,
    ) {
        let new_size = if size.is_none() {
            vk::WHOLE_SIZE
        } else {
            size.unwrap()
        };

        let new_offset = if offset.is_none() { 0 } else { offset.unwrap() };

        let mapped_range = [vk::MappedMemoryRange {
            s_type: vk::StructureType::MAPPED_MEMORY_RANGE,
            p_next: std::ptr::null(),
            memory: self.memory,
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

    pub fn get_buffer(&self) -> vk::Buffer {
        return self.buffer;
    }

    pub fn descriptor_info(
        &self,
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
    ) -> vk::DescriptorBufferInfo {
        let new_size = if size.is_none() {
            vk::WHOLE_SIZE
        } else {
            size.unwrap()
        };

        let new_offset = if offset.is_none() { 0 } else { offset.unwrap() };

        return vk::DescriptorBufferInfo {
            buffer: self.buffer,
            offset: new_offset,
            range: new_size,
        };
    }
}

