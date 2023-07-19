mod game;
mod window;
mod device;
mod swapchain;
mod pipeline;

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

fn main() {
    println!("Hello World");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    
    let mut window = Window::new(&mut glfw,"Revier:DEV BUILD #1",640,480);
    let device = Device::new(&window,&glfw);
    let swapchain = Swapchain::new(&device,window.getExtent());
    let pipeline_config = PipelineConfiguration::default();
    let pipeline = Pipeline::new(&device,"/shaders/simple_shader.vert.spv","/shaders/simple_shader.frag.spv",pipeline_config);

    window._window.set_key_polling(true);

    while !window._window.should_close() {

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.events) {
            
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window._window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}
