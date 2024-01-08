use lumina_core::device::Device;

use ash::vk;
use std::{ffi::c_void, rc::Rc};


#[derive(Clone)]
pub struct Buffer {
    device: Option<Rc<Device>>,
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    mapped: *mut c_void,
    buffer_size: u64,
}

impl Buffer {
    pub fn new(
        device: Rc<Device>,
        instance_size: u64,
        instance_count: u64,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> Self {
        let buffer_size = instance_size * instance_count;

        let (vertex_buffer, vertex_buffer_memory) =
            device.create_buffer(buffer_size, usage, properties);

        Self {
            device: Some(device),
            buffer: vertex_buffer,
            memory: vertex_buffer_memory,
            mapped: std::ptr::null_mut(),
            buffer_size: buffer_size,
        }
    }

    pub fn default() -> Self{
        return Self { device: None, buffer: vk::Buffer::null(), memory: vk::DeviceMemory::null(), mapped: std::ptr::null_mut(), buffer_size: 0 };
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

    pub fn map(
        &mut self,
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
    ) {
        let new_size = size.unwrap_or(vk::WHOLE_SIZE);
        let new_offset = offset.unwrap_or(0);

        assert!(self.mapped.is_null(), "Memory is already mapped"); // Check if memory is already mapped

        unsafe {
            self.mapped = self.device.as_ref().unwrap()
                .device()
                .map_memory(
                    self.memory,
                    new_offset,
                    new_size,
                    vk::MemoryMapFlags::empty(),
                )
                .expect("Failed to map memory on the buffer!");
        }
    }

    pub fn write_to_buffer<T>(
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
            let mem_offset = (self.mapped as *mut u8).offset(new_offset as isize);
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const c_void,
                mem_offset as *mut c_void,
                new_size as usize,
            );
        }
    }

    pub fn unmap(&self) {
        unsafe {
            self.device.as_ref().unwrap().device().unmap_memory(self.memory);
        }
    }

    pub fn flush(
        &self,
        size: Option<vk::DeviceSize>,
        offset: Option<vk::DeviceSize>,
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
            return self.device.as_ref().unwrap()
                .device()
                .flush_mapped_memory_ranges(&mapped_range)
                .expect("Failed to flush memory from the buffer!");
        }
    }


    pub fn get_buffer_size(&self) -> u64 {
        return self.buffer_size;
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

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { 
            self.device.as_ref().unwrap().device().device_wait_idle().unwrap();
            self.device.as_ref().unwrap().device().destroy_buffer(self.buffer, None);
            self.device.as_ref().unwrap().device().free_memory(self.memory, None);
        };
    }
}
