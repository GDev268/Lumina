use ash::vk;
use std::os::raw::c_void;
use std::ffi::CStr;

use ash::extensions::{
    ext::DebugUtils
};

unsafe extern "system" fn debugCallback(
    messageSeverity:vk::DebugUtilsMessageSeverityFlagsEXT,
    messageType:vk::DebugUtilsMessageTypeFlagsEXT,
    pCallbackData:*const vk::DebugUtilsMessengerCallbackDataEXT,
    pUserData:*mut c_void,)
{
    let message = CStr::from_ptr((*pCallbackData).p_message);
}

pub struct Device{
    pub debugUtils:DebugUtils,
    pub enableValidationLayers:bool,

}

struct SwapChainSupportDetails{
    surfaceCapabilities:vk::SurfaceCapabilitiesKHR,
    surfaceFormats:Vec<vk::SurfaceFormatKHR>,
    presentModes:Vec<vk::PresentModeKHR>,
}

struct QueueFamily{
    graphicsFamily:u32,
    presentFamily:u32,
    graphicsValue:bool,
    presentValue:bool,
}

impl QueueFamily {
    fn isComplete(&self) -> bool{
        return self.graphicsValue && self.presentValue;
    }
}

impl Device{
    pub fn new() {
        let enableValidationLayers:bool = true;

    }

    pub fn createInstance(){

    }

    pub fn setupDebugMessenger(){

    }

    pub fn createSurface(){

    }

    pub fn pickPhysicalDevice(){

    }


    pub fn createLogicalDevice(){

    }

    pub fn createCommandPool(){

    }

    pub fn getVulkanVersion(){

    }
    
    pub fn isDeviceSuitable(physicalDevice:vk::PhysicalDevice) -> bool{
        return false;
    }

    pub fn checkValidationLayerSupport() -> bool{
        return false;
    }

    pub fn populateDebugMessengerCreateInfo(createInfo:vk::DebugUtilsMessengerCreateInfoEXT){

    } 

    pub fn checkDeviceExtensionSupport(physicalDevice:vk::PhysicalDevice) -> bool{
        return false;
    }

}

/*         
        void createInstance();
        void setupDebugMessenger();
        void createSurface();
        void pickPhysicalDevice();
        void createLogicalDevice();
        void createCommandPool();
        void getVulkanVersion();

        bool isDeviceSuitable(VkPhysicalDevice physicalDevice);
        bool checkValidationLayerSupport();
        void populateDebugMessengerCreateInfo(VkDebugUtilsMessengerCreateInfoEXT &createInfo);
        bool checkDeviceExtensionSupport(VkPhysicalDevice physicalDevice);

        std::vector<const char *> getRequiredExtensions();*/