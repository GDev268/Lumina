use crate::window::Window;
use ash::{vk, Entry};
use cgmath::Zero;
use raw_window_handle::HasRawDisplayHandle;
use sprintf::sprintf;
use std::collections::BTreeSet;
use std::ffi::{c_char, CStr, CString};
use std::os::raw::c_void;
use std::ptr::{self};
use std::rc::Rc; 

use ash::extensions::ext::DebugUtils;

unsafe extern "system" fn vulkan_debug_callback(
    _message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let _message = CStr::from_ptr((*p_callback_data).p_message);

    return vk::FALSE;
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

pub struct SurfaceKHR {
    pub surface_loader: ash::extensions::khr::Surface,
    pub _surface: vk::SurfaceKHR,
}

struct SwapChainSupportDetails {
    surface_capabilities: Option<vk::SurfaceCapabilitiesKHR>,
    surface_formats: Option<Vec<vk::SurfaceFormatKHR>>,
    present_modes: Option<Vec<vk::PresentModeKHR>>,
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
    pub _device: Option<ash::Device>,
    pub surface: Option<SurfaceKHR>,
    pub graphics_queue: Option<vk::Queue>,
    pub present_queue: Option<vk::Queue>,
    pub physical_device: Option<vk::PhysicalDevice>,
    pub instance: Option<ash::Instance>,
    pub debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub entry: Option<ash::Entry>,
    pub game_version: Option<u32>,
    pub num_devices: i32,
}

impl Device {
    pub fn new(window:&Window) -> Device {
        let enable_validation_layers: bool = true;
        let mut device: Device = Device::default(enable_validation_layers);

        Device::create_instance(&mut device,window);
        device.debug_messenger = Device::setup_debug_messenger(&mut device);
        device.surface = Device::create_surface(&mut device,window);
        Device::pick_physical_device(&mut device);
        Device::create_logical_device(&mut device);
        Device::create_command_pool(&mut device);
        Device::get_vulkan_version(&mut device);

        return device;
    }


    pub fn default(enable_validation: bool) -> Device {
        return Device {
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
            entry: None,
            game_version: None,
            num_devices: 0,
        };
    }

    pub fn cleanup() {}

    pub fn create_buffer(
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
        buffer: &vk::Buffer,
        buffer_memory: &vk::DeviceMemory,
    ) {
    }

    pub fn begin_single_time_commands() /*-> vk::CommandBuffer*/ {}

    pub fn end_single_time_commands() {}

    pub fn copy_buffer(src_buffer: vk::Buffer, dst_buffer: vk::Buffer, size: vk::DeviceSize) {}

    pub fn find_memory_type(filter: u32, properties: vk::MemoryPropertyFlags) -> u32 {
        return 0;
    }

    fn create_instance(self: &mut Device,window:&Window) {
        let entry = Entry::linked();
        if self.enable_validation_layers && !self.check_validation_layer_support(&entry) {
            panic!("validation layers requested, but not available!");
        }

        let app_name = CString::new("Revier Engine").unwrap();
        let engine_name = CString::new("Revier").unwrap();

        let app_info: vk::ApplicationInfo = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: ash::vk::make_api_version(0, 1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: ash::vk::make_api_version(0, 1, 0, 0),
            api_version: ash::vk::make_api_version(0, 1, 0, 0),
        };

        let mut create_info: vk::InstanceCreateInfo = vk::InstanceCreateInfo::default();
        create_info.s_type = vk::StructureType::INSTANCE_CREATE_INFO;
        create_info.p_application_info = &app_info;

        let extensions = self.get_required_extensions(window);
        create_info.enabled_extension_count = extensions.len() as u32;
        create_info.pp_enabled_extension_names = extensions.as_ptr();

        let c_layers: Vec<std::ffi::CString> = VALIDATION_LAYERS
            .iter()
            .map(|&s| {
                std::ffi::CString::new(s)
                    .expect("Failed to convert 'VALIDATION_LAYERS' to an C String")
            })
            .collect();

        let pointer_layers: Vec<*const i8> = c_layers.iter().map(|cl| cl.as_ptr()).collect();

        if self.enable_validation_layers {
            create_info.enabled_layer_count = VALIDATION_LAYERS.len() as u32;
            create_info.pp_enabled_layer_names = pointer_layers.as_ptr();
        }

        let debug_create_info = self.populate_debug_messenger_create_info();

        self.instance = Some(unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        });

        self.entry = Some(entry);

        println!("Required Extensions:");
        for i in 0..extensions.len() {
            let word = convert_vk_to_string_const(extensions[i]);
            println!("\t{}", word);
        }
    }

    fn setup_debug_messenger(self: &mut Device) -> Option<vk::DebugUtilsMessengerEXT> {
        if !self.enable_validation_layers {
            return None;
        }

        if self.entry.is_none() {
            return Some(vk::DebugUtilsMessengerEXT::default());
        }

        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(
            self.entry.as_ref().unwrap(),
            &self.instance.as_ref().unwrap(),
        );

        unsafe {
            return Some(
                debug_utils_loader
                    .create_debug_utils_messenger(&self.populate_debug_messenger_create_info(), None)
                    .expect("Failed to create an Debug Messenger"),
            );
        }
    }

    fn create_surface(self: &mut Device,window:&Window) -> Option<SurfaceKHR> {
        let surface: SurfaceKHR = SurfaceKHR {
            surface_loader: ash::extensions::khr::Surface::new(
                self.entry.as_ref().unwrap(),
                self.instance.as_ref().unwrap(),
            ),
            _surface: window.createWindowSurface(
                self.instance.as_ref().unwrap(),
                self.entry.as_ref().unwrap(),
            ),
        };
        return Some(surface);
    }

    fn pick_physical_device(self: &mut Device) {
        unsafe {
            let physical_devices = self.instance.as_ref().unwrap().enumerate_physical_devices();

            for physical_device in physical_devices.unwrap().iter() {
                if Device::is_device_suitable(self, &physical_device) {
                    self.physical_device = Some(*physical_device);
                }
            }

            self.physical_device_properties = Some(
                self.instance
                    .as_ref()
                    .unwrap()
                    .get_physical_device_properties(self.physical_device.unwrap()),
            );
        }
    }

    fn create_logical_device(self: &mut Device) {
        let indices: QueueFamily = self.find_queue_families(&self.physical_device.unwrap());

        let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = Vec::new();

        let unique_queue_families: BTreeSet<u32> =
            vec![indices.graphics_family, indices.present_family]
                .into_iter()
                .collect(); 

        let queue_priority: *const f32 = &1.0;

        for queue_family in unique_queue_families {
            let mut queue_create_info: vk::DeviceQueueCreateInfo =
                vk::DeviceQueueCreateInfo::default();

            queue_create_info.s_type = vk::StructureType::DEVICE_QUEUE_CREATE_INFO;
            queue_create_info.queue_family_index = queue_family;
            queue_create_info.queue_count = 1;
            queue_create_info.p_queue_priorities = queue_priority;
            queue_create_infos.push(queue_create_info);
        }

        let mut device_features: vk::PhysicalDeviceFeatures = vk::PhysicalDeviceFeatures::default();
        device_features.sampler_anisotropy = vk::TRUE;

        let mut create_info: vk::DeviceCreateInfo = vk::DeviceCreateInfo::default();
        create_info.s_type = vk::StructureType::DEVICE_CREATE_INFO;
        create_info.queue_create_info_count = queue_create_infos.len() as u32;
        create_info.p_queue_create_infos = queue_create_infos.as_ptr();
        create_info.p_enabled_features = &device_features;
        create_info.enabled_extension_count = DEVICE_EXTENSIONS.len() as u32;

        //Convert the DEVICE_EXTENSIONS([&'static str;n]) to an *const i8(or c_char)
        let mut c_extensions: Vec<Vec<u8>> = Vec::with_capacity(DEVICE_EXTENSIONS.len());

        for string in DEVICE_EXTENSIONS {
            c_extensions.push(CString::new(string).unwrap().into_bytes_with_nul());
        }

        let pointers: Vec<*const i8> = c_extensions
            .iter()
            .map(|s| s.as_ptr() as *const i8)
            .collect();

        create_info.pp_enabled_extension_names = pointers.as_ptr();

        unsafe {
            self._device = Some(
                self.instance
                    .as_ref()
                    .unwrap()
                    .create_device(self.physical_device.unwrap(), &create_info, None)
                    .expect("Failed to create logical Device!"),
            );

            self.graphics_queue = Some(
                self._device
                    .as_ref()
                    .unwrap()
                    .get_device_queue(indices.graphics_family, 0),
            );
            self.present_queue = Some(
                self._device
                    .as_ref()
                    .unwrap()
                    .get_device_queue(indices.present_family, 0),
            );
        }
    }

    fn create_command_pool(self: &mut Device) {}

    pub fn get_vulkan_version(self: &mut Device) {
        self.game_version = self
            .entry
            .as_ref()
            .unwrap()
            .try_enumerate_instance_version()
            .unwrap();

        let driver_version = Device::get_driver_version(
            self.physical_device_properties.unwrap().driver_version,
            self.physical_device_properties.unwrap().vendor_id,
        );
        let graphics_card: &str;
        unsafe {
            graphics_card = CStr::from_ptr(
                self.physical_device_properties
                    .unwrap()
                    .device_name
                    .as_ptr(),
            )
            .to_str()
            .unwrap();
        }

        println!("\n======= VULKAN INFO =======");
        println!(
            "API Version: {:?}.{:?}.{:?}",
            vk::api_version_major(self.game_version.unwrap()),
            vk::api_version_minor(self.game_version.unwrap()),
            vk::api_version_patch(self.game_version.unwrap())
        );
        println!("Driver Version: {}", driver_version);
        println!("Device Count: {:?}", self.num_devices);
        println!("Graphics Card: {}", graphics_card);
        println!(
            "Physical device ID: {:?}",
            self.physical_device_properties.unwrap().device_id
        );

        match self.physical_device_properties.unwrap().device_type {
            vk::PhysicalDeviceType::OTHER => println!("Graphics device Type: OTHER"),
            vk::PhysicalDeviceType::INTEGRATED_GPU => {
                println!("Graphics device Type: INTEGRATED GPU")
            }
            vk::PhysicalDeviceType::DISCRETE_GPU => println!("Graphics device Type: DISCRETE GPU"),
            vk::PhysicalDeviceType::VIRTUAL_GPU => println!("Graphics device Type: VIRTUAL GPU"),
            vk::PhysicalDeviceType::CPU => println!("Graphics device Type: CPU"),
            _ => panic!("Physical Device Type Not existent"),
        };
        println!(
            "Vendor ID: {:?}",
            self.physical_device_properties.unwrap().vendor_id
        );
        println!("============================\n");
    }

    fn is_device_suitable(self: &mut Device, physical_device: &vk::PhysicalDevice) -> bool {
        let indices: QueueFamily = Device::find_queue_families(self, physical_device);

        let extensions_supported = self.check_device_extension_support(*physical_device);

        let mut swapchain_adequate = false;
        if extensions_supported {
            let swapchain_support: SwapChainSupportDetails =
                self.query_swapchain_support(physical_device);

            swapchain_adequate = !swapchain_support.surface_formats.unwrap().is_empty()
                && !swapchain_support.present_modes.unwrap().is_empty();
        }

        unsafe {
            let supported_features = self
                .instance
                .as_ref()
                .unwrap()
                .get_physical_device_features(*physical_device);

            return indices.isComplete()
                && extensions_supported
                && swapchain_adequate
                && !supported_features.sampler_anisotropy.is_zero();
        }
    }

    fn check_validation_layer_support(&mut self, entry: &ash::Entry) -> bool {
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
                return false;
            }
        }

        return true;
    }

    fn populate_debug_messenger_create_info(&self) -> vk::DebugUtilsMessengerCreateInfoEXT {
        return vk::DebugUtilsMessengerCreateInfoEXT {
            s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            pfn_user_callback: Some(vulkan_debug_callback),
            p_user_data: ptr::null_mut(),
            p_next: ptr::null(),
            flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        };
    }

    fn check_device_extension_support(&self, physical_device: vk::PhysicalDevice) -> bool {
        unsafe {
            let available_extensions = self
                .instance
                .as_ref()
                .unwrap()
                .enumerate_device_extension_properties(physical_device)
                .unwrap();

            let mut required_extensions: Vec<&str> = Vec::new();

            for ext in DEVICE_EXTENSIONS {
                required_extensions.push(ext);
            }

            for device_extension in available_extensions {
                let new_extension = CStr::from_ptr(device_extension.extension_name.as_ptr())
                    .to_str()
                    .unwrap();

                required_extensions.retain(|&extension| extension != new_extension);
            }

            return required_extensions.is_empty();
        }
    }

    fn get_required_extensions(&self,window:&Window) -> Vec<*const i8> {
        let mut extensions = ash_window::enumerate_required_extensions(
            window._window.raw_display_handle(),
        )
        .unwrap()
        .to_vec();

        if self.enable_validation_layers {
            extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr());
        }

        return extensions;
    }

    fn find_queue_families(self: &mut Device, physical_device: &vk::PhysicalDevice) -> QueueFamily {
        let mut indices: QueueFamily = QueueFamily {
            graphics_family: 0,
            present_family: 0,
            graphics_value: false,
            present_value: false,
        };

        let mut i: i32 = 0;
        unsafe {
            let queue_families = self
                .instance
                .as_ref()
                .unwrap()
                .get_physical_device_queue_family_properties(*physical_device);

            for queue_family in queue_families {
                if queue_family.queue_count > 0
                    && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                {
                    indices.graphics_family = i as u32;
                    indices.graphics_value = true;
                }

                let present_support = self
                    .surface
                    .as_ref()
                    .unwrap()
                    .surface_loader
                    .get_physical_device_surface_support(
                        *physical_device,
                        i as u32,
                        self.surface.as_ref().unwrap()._surface,
                    )
                    .unwrap();

                if queue_family.queue_count > 0 && present_support {
                    indices.present_family = i as u32;
                    indices.present_value = true;
                }

                if indices.isComplete() {
                    break;
                }

                i += 1;
            }

            return indices;
        }
    }

    fn query_swapchain_support(
        &self,
        physical_device: &vk::PhysicalDevice,
    ) -> SwapChainSupportDetails {
        let mut details: SwapChainSupportDetails = SwapChainSupportDetails {
            surface_capabilities: None,
            surface_formats: None,
            present_modes: None,
        };

        unsafe {
            details.surface_capabilities = Some(
                self.surface
                    .as_ref()
                    .unwrap()
                    .surface_loader
                    .get_physical_device_surface_capabilities(
                        *physical_device,
                        self.surface.as_ref().unwrap()._surface,
                    )
                    .unwrap(),
            );

            details.surface_formats = Some(
                self.surface
                    .as_ref()
                    .unwrap()
                    .surface_loader
                    .get_physical_device_surface_formats(
                        *physical_device,
                        self.surface.as_ref().unwrap()._surface,
                    )
                    .unwrap(),
            );

            details.present_modes = Some(
                self.surface
                    .as_ref()
                    .unwrap()
                    .surface_loader
                    .get_physical_device_surface_present_modes(
                        *physical_device,
                        self.surface.as_ref().unwrap()._surface,
                    )
                    .unwrap(),
            )
        }

        return details;
    }

    #[cfg(all(unix, not(target_os = "windows")))]
    fn get_driver_version(version_raw: u32, vendor_id: u32) -> String {
        //FOR NVIDIA GRAPHICS CARDS
        if vendor_id == 4318 {
            return sprintf!(
                "%d.%d.%d.%d",
                version_raw >> 22 & 0x3ff,
                version_raw >> 14 & 0x0ff,
                version_raw >> 6 & 0x0ff,
                version_raw & 0x003f
            )
            .unwrap();
        }
        //DEFAULT
        else {
            return sprintf!(
                "%d.%d.%d",
                version_raw >> 22,
                version_raw >> 12 & 0x3ff,
                version_raw & 0xfff
            )
            .unwrap();
        }
    }

    #[cfg(all(target_os = "windows"))]
    fn get_driver_version(version_raw: u32, vendor_id: u32) -> String {
        //FOR WINDOWS
        if vendor_id == 0x8086 {
            return sprintf!("%d.%d", version_raw >> 14, version_raw & 0x3fff);
        }
        //DEFAULT
        else {
            return sprintf!(
                "%d.%d.%d",
                version_raw >> 22,
                version_raw >> 12 & 0x3ff,
                version_raw & 0xfff
            )
            .unwrap();
        }
    }
}
