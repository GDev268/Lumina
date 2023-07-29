mod data;
mod graphics;
mod engine;

use graphics::pipeline::{Pipeline, PipelineConfiguration};
use graphics::renderer::Renderer;
use engine::{device::Device, swapchain::Swapchain,window::Window};
use glfw::{self};


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let window = Window::new(&mut glfw, "Hello Vulkan!", 800, 640);
    let device = Device::new(&window, &glfw);
 
    while !window._window.should_close() {
        glfw.poll_events();
        
    }
}
