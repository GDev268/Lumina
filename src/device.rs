use ash::{vk, Entry};
use std::ffi::CStr;
use std::os::raw::c_void;

use ash::extensions::ext::DebugUtils;

unsafe extern "system" fn debugCallback(
    _message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) {
    let _message = CStr::from_ptr((*p_callback_data).p_message);
}

const VALIDATION_LAYERS:[&'static str;1] = ["VK_LAYER_KHRONOS_validation"];
struct SwapChainSupportDetails {
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

struct QueueFamily {
    graphics_family: u32,
    present_family: u32,
    graphics_value: bool,
    present_value: bool,
}

impl QueueFamily {
    fn isComplete(&self) -> bool {
        return self.graphics_value && self.present_value;
    }
}

pub struct Device {
    pub debug_utils: Option<DebugUtils>,
    pub enable_validation_layers: bool,
    pub physical_device_properties: Option<vk::PhysicalDeviceProperties>,
    pub command_pool: Option<vk::CommandPool>,
    pub _device: Option<vk::Device>,
    pub surface: Option<vk::SurfaceKHR>,
    pub graphics_queue: Option<vk::Queue>,
    pub present_queue: Option<vk::Queue>,
    pub physical_device: Option<vk::PhysicalDevice>,
}

impl Device {
    pub fn new() {
        let enable_validation_layers: bool = true;
        let mut device: Device = Device::none(enable_validation_layers);

        Device::createInstance(&mut device);
        Device::setupDebugMessenger(&mut device);
        Device::createSurface(&mut device);
        Device::pickPhysicalDevice(&mut device);
        Device::createLogicalDevice(&mut device);
        Device::getVulkanVersion(&mut device);
    }

    pub fn none(enable_validation: bool) -> Self {
        return Self {
            debug_utils: None,
            enable_validation_layers: enable_validation,
            physical_device_properties: None,
            command_pool: None,
            _device: None,
            surface: None,
            graphics_queue: None,
            present_queue: None,
            physical_device: None,
        };
    }

    pub fn cleanup() {}

    pub fn createBuffer(
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
        buffer: &vk::Buffer,
        buffer_memory: &vk::DeviceMemory,
    ) {
    }

    pub fn beginSingleTimeCommands() /*-> vk::CommandBuffer*/ {}

    pub fn endSingleTimeCommands() {}

    pub fn copyBuffer(src_buffer: vk::Buffer, dst_buffer: vk::Buffer, size: vk::DeviceSize) {}

    pub fn findMemoryType(filter: u32, properties: vk::MemoryPropertyFlags) -> u32 {
        return 0;
    }

    fn createInstance(self: &mut Device) {}

    fn setupDebugMessenger(self: &mut Device) {}

    fn createSurface(self: &mut Device) {}

    fn pickPhysicalDevice(self: &mut Device) {}

    fn createLogicalDevice(self: &mut Device) {}

    fn createCommandPool(self: &mut Device) {}

    fn getVulkanVersion(self: &mut Device) {}

    fn isDeviceSuitable(_physical_device: vk::PhysicalDevice) -> bool {
        return false;
    }

    fn checkValidationLayerSupport(entry: &ash::Entry) -> bool {
        let layer_properties = entry.enumerate_instance_layer_properties().unwrap();

        return false;
    }

    fn populateDebugMessengerCreateInfo(_create_info: vk::DebugUtilsMessengerCreateInfoEXT) {}

    fn checkDeviceExtensionSupport(_physical_device: vk::PhysicalDevice) -> bool {
        return false;
    }
}
