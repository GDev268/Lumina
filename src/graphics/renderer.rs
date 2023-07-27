use crate::engine::{swapchain::{Swapchain, self}, window::Window, device::Device};
use ash::vk;

struct Renderer{
    swapchain:Swapchain,
    command_buffers:Vec<vk::CommandBuffer>,
    current_image_index:u32,
    current_frame_index:i32,
    is_frame_started:bool
}

impl Renderer {
    pub fn new(window:&Window,device:&Device){

    }
    
    pub fn get_swapchain_renderpass(&self) -> vk::RenderPass{
        return self.swapchain.get_renderpass();
    }

    pub fn get_aspect_ratio(&self) -> f64{
        return self.get_aspect_ratio();
    } 

    pub fn is_frame_in_progress(&self) -> bool {
        return self.is_frame_started;
    }

    pub fn get_current_command_buffer(&self) -> vk::CommandBuffer {
        assert!(self.is_frame_started, "Cannot get command buffer when frame not in progress");

        return self.command_buffers[self.current_frame_index as usize];
    }

    pub fn get_frame_index(&self) -> i32 {
        assert!(self.is_frame_started, "Cannot get frame index when frame not in progress");

        return self.current_frame_index;
    }

    pub fn begin_frame(&self) -> vk::CommandBuffer{
        return vk::CommandBuffer::null();
    }

    pub fn end_frame(&self){

    }

    pub fn begin_swapchain_renderpass(&self,command_buffer:vk::CommandBuffer){
        
    }
}