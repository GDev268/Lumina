use crate::window::Window;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use ash::vk::LayerProperties;
use ash::{vk, Entry};
use winit::event_loop::EventLoop;
use std::borrow::{BorrowMut, Borrow};
use std::ffi::{c_char, CStr,CString};
use std::ops::Deref;
use std::os::raw::c_void;
use std::ptr;

use ash::extensions::ext::DebugUtils;

unsafe extern "system" fn debugCallback(
    _message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) {
    let _message = CStr::from_ptr((*p_callback_data).p_message);
}

pub fn convert_vk_to_string(string: &[c_char]) -> &str {
    unsafe {
        let pointer = string.as_ptr();
        return CStr::from_ptr(pointer).to_str().unwrap();
    }
}

pub fn convert_vk_to_string_const(string: *const c_char) -> &'static str {
    unsafe {
        return CStr::from_ptr(string).to_str().unwrap();
    }
}

const VALIDATION_LAYERS: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];
const DEVICE_EXTENSIONS: [&'static str; 1] = ["VK_KHR_swapchain"];
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
    instance: Option<vk::Instance>,
    debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub window: Option<Window>,
    game_version: u32,
    num_devices: i32,
}

impl Device {
    pub fn new(window:Window) -> Self {
        let enable_validation_layers: bool = true;
        let mut device: Device = Device::none(enable_validation_layers);
        

        device.window = Some(window);
        Device::createInstance(&mut device);
        Device::setupDebugMessenger(&mut device);
        Device::createSurface(&mut device);
        Device::pickPhysicalDevice(&mut device);
        Device::createLogicalDevice(&mut device);
        Device::getVulkanVersion(&mut device);

        return device;
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
            instance: None,
            debug_messenger: None,
            window: None,
            game_version: 0,
            num_devices: 0,
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

    fn createInstance(self: &mut Device) {
        let entry = Entry::linked();
        if self.enable_validation_layers && !self.checkValidationLayerSupport(&entry) {
            panic!("validation layers requested, but not available!");
        }

        let app_name = CString::new("Revier Engine").unwrap();
        let engine_name = CString::new("Revier").unwrap();

        let app_info:vk::ApplicationInfo = vk::ApplicationInfo { 
            s_type: vk::StructureType::APPLICATION_INFO, 
            p_next: ptr::null(), 
            p_application_name: app_name.as_ptr(), 
            application_version: ash::vk::make_api_version(0, 1, 0,0), 
            p_engine_name: engine_name.as_ptr(), 
            engine_version: ash::vk::make_api_version(0, 1, 0,0), 
            api_version: ash::vk::make_api_version(0, 1, 0,0), 
        };

        let mut create_info:vk::InstanceCreateInfo = vk::InstanceCreateInfo::default();
        create_info.s_type = vk::StructureType::INSTANCE_CREATE_INFO;
        create_info.p_application_info = &app_info;
        
        let extensions = self.getRequiredExtensions();
        create_info.enabled_extension_count = extensions.len() as u32;
        create_info.pp_enabled_extension_names = extensions.as_ptr();
       
        let c_layers:Vec<std::ffi::CString> = VALIDATION_LAYERS
        .iter()
        .map(|&s| std::ffi::CString::new(s).expect("Failed to convert 'VALIDATION_LAYERS' to an C String"))
        .collect();

        let pointer_layers:Vec<*const i8> = c_layers.iter()
        .map(|cl| cl.as_ptr())
        .collect();

        if self.enable_validation_layers {
            create_info.enabled_layer_count = VALIDATION_LAYERS.len() as u32;
            create_info.pp_enabled_layer_names = pointer_layers.as_ptr();
        }

        let debug_create_info:vk::DebugUtilsMessengerCreateInfoEXT = vk::DebugUtilsMessengerCreateInfoEXT::default();

        self.populateDebugMessengerCreateInfo(debug_create_info);

        println!("Required Extensions:");
        for i in 0..extensions.len(){
            let word = convert_vk_to_string_const(extensions[i]);
            println!("\t{}",word);
        }

    }

    fn setupDebugMessenger(self: &mut Device) {}

    fn createSurface(self: &mut Device) {}

    fn pickPhysicalDevice(self: &mut Device) {}

    fn createLogicalDevice(self: &mut Device) {}

    fn createCommandPool(self: &mut Device) {}

    fn getVulkanVersion(self: &mut Device) {}

    fn isDeviceSuitable(_physical_device: vk::PhysicalDevice) -> bool {
        return false;   
    }

    fn checkValidationLayerSupport(&mut self,entry: &ash::Entry) -> bool {
        let layer_properties = entry.enumerate_instance_layer_properties().unwrap();
        let mut layer_found: bool = false;

        for validation_layer in VALIDATION_LAYERS {
            for layer in layer_properties.iter() {
                let layer_name = convert_vk_to_string(&layer.layer_name);
                if layer_name == validation_layer {
                    layer_found = true;
                    break;
                }
            }
            
            if !layer_found {
                return false
            }
        }
        

        return true;
    }

    fn populateDebugMessengerCreateInfo(&self,_create_info: vk::DebugUtilsMessengerCreateInfoEXT) {}

    fn checkDeviceExtensionSupport(_physical_device: vk::PhysicalDevice) -> bool {
        return false;
    }

    fn getRequiredExtensions(&self) -> Vec<*const i8> {
        let window = self.window.as_ref().unwrap();
        let mut extensions = ash_window::enumerate_required_extensions(window._window.raw_display_handle()).unwrap().to_vec();

        if self.enable_validation_layers {
            extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());
        }
        
       return extensions;
    }
}