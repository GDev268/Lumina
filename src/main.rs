mod data;
mod graphics;
mod engine;

use graphics::pipeline::{Pipeline, PipelineConfiguration};
use engine::{device::Device, swapchain::Swapchain,window::Window};
use ash::vk;
use data::buffer::Buffer;
use glfw::{self};
use glfw::{Action, Context, Key};
use simple_logger::SimpleLogger;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let window = Window::new(&mut glfw, "Hello Vulkan!", 800, 640);
    let device = Device::new(&window, &glfw);
    let swapchain = Swapchain::new(&device, window.get_extent());
 
    while !window._window.should_close() {
        glfw.poll_events();
        
    }
}
