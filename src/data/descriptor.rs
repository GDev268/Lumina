use ash::{vk, Device};
use std::{collections::HashMap, hash};

struct Descriptor {
    bindings: HashMap<u32, vk::DescriptorSetLayoutBinding>,
}

impl Descriptor {
    pub fn add_binding(
        binding: u32,
        descriptor_type: vk::DescriptorType,
        stage_flags: vk::ShaderStageFlags,
        count: u32,
        hashmap: Option<HashMap<u32, vk::DescriptorSetLayoutBinding>>,
    ) -> HashMap<u32, vk::DescriptorSetLayoutBinding> {
        let mut hashmap: HashMap<u32, vk::DescriptorSetLayoutBinding> = if hashmap.is_none() {
            HashMap::new()
        } else {
            hashmap.unwrap()
        };

        let layout_binding = vk::DescriptorSetLayoutBinding {
            binding: binding,
            descriptor_type: descriptor_type,
            descriptor_count: count,
            stage_flags: stage_flags,
            p_immutable_samplers: std::ptr::null(),
        };

        hashmap.insert(binding, layout_binding);

        return hashmap;
    }

    pub fn build(
        device: &Device,
        bindings: HashMap<u32, vk::DescriptorSetLayoutBinding>,
    ) -> Descriptor {
        return Descriptor::new(device, bindings);
    }

    pub fn new(device: &Device, bindings: HashMap<u32, vk::DescriptorSetLayoutBinding>) -> Self {
        let set_layout_bindings: Vec<vk::DescriptorSetLayoutBinding> =
            bindings.keys().map(|f| *bindings.get(f).unwrap()).collect();
    }
}

struct DescriptorPool {}

impl DescriptorPool {}

struct DescriptorWriter {}

impl DescriptorWriter {}
