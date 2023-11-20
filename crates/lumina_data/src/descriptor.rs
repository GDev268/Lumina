use lumina_core::device::Device;

use ash::vk;
use std::collections::HashMap;

pub struct DescriptorSetLayout {
    bindings: HashMap<u32, vk::DescriptorSetLayoutBinding>,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
}

impl DescriptorSetLayout {
    pub fn new(device: &Device, bindings: HashMap<u32, vk::DescriptorSetLayoutBinding>) -> Self {
        let mut set_layout_bindings: Vec<vk::DescriptorSetLayoutBinding> = Vec::new();

        bindings.iter().for_each(|(_, binding)| {
            set_layout_bindings.push(*binding);
        });

        let descriptor_set_layout_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: set_layout_bindings.len() as u32,
            p_bindings: set_layout_bindings.as_ptr(),
        };

        let descriptor_set_layout = unsafe {
            device
                .device()
                .create_descriptor_set_layout(&descriptor_set_layout_info, None)
                .unwrap()
        };

        Self {
            bindings,
            descriptor_set_layout,
        }
    }

    pub fn get_descriptor_set_layout(&self) -> vk::DescriptorSetLayout {
        return self.descriptor_set_layout;
    }
}

impl Default for DescriptorSetLayout {
    fn default() -> Self{
        return Self { descriptor_set_layout: vk::DescriptorSetLayout::null(), bindings: HashMap::new() };
    }
}

pub struct LayoutConfig {
    bindings: HashMap<u32, vk::DescriptorSetLayoutBinding>,
}

impl LayoutConfig {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::<u32, vk::DescriptorSetLayoutBinding>::new(),
        }
    }

    pub fn add_binding(
        &mut self,
        binding: u32,
        descriptor_type: vk::DescriptorType,
        stage_flags: vk::ShaderStageFlags,
        count: u32,
    ) {
        assert!(
            !self.bindings.contains_key(&binding),
            "Binding already in use"
        );

        let layout_binding = vk::DescriptorSetLayoutBinding {
            binding: binding,
            descriptor_type: descriptor_type,
            descriptor_count: count,
            stage_flags: stage_flags,
            p_immutable_samplers: std::ptr::null(),
        };

        self.bindings.insert(binding, layout_binding);
    }

    pub fn build(&self, device: &Device) -> DescriptorSetLayout {
        DescriptorSetLayout::new(device, HashMap::clone(&self.bindings))
    }
}



pub struct PoolConfig {
    pool_sizes: Vec<vk::DescriptorPoolSize>,
    max_sets: u32,
    pool_flags: vk::DescriptorPoolCreateFlags,
}

impl PoolConfig {
    pub fn new() -> Self {
        return Self {
            pool_sizes: Vec::new(),
            max_sets: 1000,
            pool_flags: vk::DescriptorPoolCreateFlags::empty(),
        };
    }

    pub fn add_pool_size(&mut self, descriptor_type: vk::DescriptorType, count: u32) {
        self.pool_sizes.push(vk::DescriptorPoolSize {
            ty: descriptor_type,
            descriptor_count: count,
        });
    }

    pub fn set_pool_flags(&mut self, flags: vk::DescriptorPoolCreateFlags) {
        self.pool_flags = flags;
    }

    pub fn set_max_sets(&mut self, sets: u32) {
        self.max_sets = sets;
    }

    pub fn build(&self, device: &Device) -> DescriptorPool {
        return DescriptorPool::new(device, self.max_sets, self.pool_flags, &self.pool_sizes);
    }
}

pub struct DescriptorPool {
    pub descriptor_pool: vk::DescriptorPool,
}

impl DescriptorPool {
    pub fn new(
        device: &Device,
        max_sets: u32,
        pool_flags: vk::DescriptorPoolCreateFlags,
        pool_sizes: &Vec<vk::DescriptorPoolSize>,
    ) -> Self {
        let descriptor_pool_info: vk::DescriptorPoolCreateInfo = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: pool_flags,
            max_sets: max_sets,
            pool_size_count: pool_sizes.len() as u32,
            p_pool_sizes: pool_sizes.as_ptr(),
        };

        let descriptor_pool: vk::DescriptorPool = unsafe {
            device
                .device()
                .create_descriptor_pool(&descriptor_pool_info, None)
                .expect("Failed to create descriptor pool")
        };

        return Self {
            descriptor_pool: descriptor_pool,
        };
    }

    pub fn allocate_descriptor(
        &self,
        device: &Device,
        descriptor_set_layout: vk::DescriptorSetLayout,
    ) -> vk::DescriptorSet {
        let alloc_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            descriptor_pool: self.descriptor_pool,
            p_set_layouts: &descriptor_set_layout,
            descriptor_set_count: 1,
        };

        unsafe {
            let result = device
                .device()
                .allocate_descriptor_sets(&alloc_info)
                .unwrap();
            return result[0];
        }
    }

    pub fn reset_pool(&self, device: &Device) {
        unsafe {
            device
                .device()
                .reset_descriptor_pool(self.descriptor_pool, vk::DescriptorPoolResetFlags::empty())
                .expect("Failed to reset descriptor pool");
        }
    }
}

pub struct DescriptorWriter {
    writers: Vec<vk::WriteDescriptorSet>,
}

impl DescriptorWriter {
    pub fn new() -> Self {
        return Self {
            writers: Vec::new(),
        };
    }

    pub fn write_buffer(
        &mut self,
        binding: u32,
        buffer_info: vk::DescriptorBufferInfo,
        set_layout: &DescriptorSetLayout,
    ) {
        assert!(
            set_layout.bindings.contains_key(&binding),
            "Layout does not contain specified binding"
        );

        let binding_description = set_layout
            .bindings
            .get(&binding)
            .expect("Failed current binding doesn't exist!");

        assert!(
            binding_description.descriptor_count == 1,
            "Binding is single descriptor info, expects multiple"
        );

        let write = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: std::ptr::null(),
            dst_set: vk::DescriptorSet::null(),
            dst_binding: binding,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: binding_description.descriptor_type,
            p_image_info: std::ptr::null(),
            p_buffer_info: &buffer_info,
            p_texel_buffer_view: std::ptr::null(),
        };

        self.writers.push(write);
    }

    pub fn write_image(
        &mut self,
        binding: u32,
        image_info: vk::DescriptorImageInfo,
        set_layout: &DescriptorSetLayout,
    ) {
        assert!(
            set_layout.bindings.len() == 1,
            "Layout doesn't contain the specified binding"
        );

        let binding_description = set_layout
            .bindings
            .get(&binding)
            .expect("Failed current binding doesn't exist!");

        assert!(
            binding_description.descriptor_count == 1,
            "Binding is single descriptor info, expects multiple"
        );

        let write = vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: std::ptr::null(),
            dst_set: vk::DescriptorSet::null(),
            dst_binding: binding,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: binding_description.descriptor_type,
            p_image_info: &image_info,
            p_buffer_info: std::ptr::null(),
            p_texel_buffer_view: std::ptr::null(),
        };

        self.writers.push(write);
    }

    pub fn build(
        &mut self,
        device: &Device,
        descriptor_set_layout: vk::DescriptorSetLayout,
        pool: &DescriptorPool,
    ) -> vk::DescriptorSet {
        let set = pool.allocate_descriptor(device, descriptor_set_layout);

        self.overwrite(device, &set);

        return set;
    }

    pub fn overwrite(&mut self, device: &Device, set: &vk::DescriptorSet) {
        self.writers.iter_mut().for_each(|writer| {
            writer.dst_set = *set;
        });

        unsafe {
            device.device().update_descriptor_sets(&self.writers, &[]);
        }
    }
}
