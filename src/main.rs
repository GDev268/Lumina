mod game;
mod window;
mod device;
mod swapchain;
mod pipeline;
mod buffer;

use std::borrow::Borrow;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use crate::pipeline::{Pipeline,PipelineConfiguration};
use crate::{device::Device, swapchain::Swapchain};
use crate::window::Window;
use simple_logger::SimpleLogger;
use glfw::{self};
use glfw::{Action, Context, Key};
use ash::vk;
use buffer::Buffer;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    
    let window = Window::new(&mut glfw,"Hello Vulkan!",800,640);
    let device = Device::new(&window,&glfw);
    let swapchain = Swapchain::new(&device,window.getExtent());
    


    while !window._window.should_close() {

        // Poll for and process events
        glfw.poll_events();
     
    }
}