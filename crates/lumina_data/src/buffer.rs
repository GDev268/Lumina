use lumina_core::device::Device;

use ash::vk;
use core::slice;
use std::{ffi::c_void, rc::Rc, sync::{Arc, Mutex}};



pub struct Buffer {
    device: Option<Arc<Device>>,
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    mapped: Arc<Mutex<Option<*mut c_void>>>,
    buffer_size: u64,
}

impl Buffer {
    pub fn new(
        device: Arc<Device>,
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
            mapped: Arc::new(Mutex::new(None)),
            buffer_size: buffer_size,
        }
    }

    pub fn default() -> Self{
        return Self { device: None, buffer: vk::Buffer::null(), memory: vk::DeviceMemory::null(), mapped: Arc::new(Mutex::new(None)), buffer_size: 0 };
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

        assert!(self.mapped.lock().unwrap().is_none(), "Memory is already mapped");

        unsafe {
            let mapped = self.device
                .as_ref()
                .unwrap()
                .device()
                .map_memory(
                    self.memory,
                    new_offset,
                    new_size,
                    vk::MemoryMapFlags::empty(),
                )
                .expect("Failed to map memory on the buffer!");
            
            *self.mapped.lock().unwrap() = Some(mapped);
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
            let mem_offset = (self.mapped.lock().unwrap().unwrap() as *mut u8).offset(new_offset as isize);
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

    pub fn convert_to_raw_data(&self) -> Vec<u8> {
        let buffer_data = unsafe {
            std::slice::from_raw_parts(self.mapped.lock().unwrap().unwrap() as *const u8, self.buffer_size as usize)
        };
        
        return buffer_data.to_vec();
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
