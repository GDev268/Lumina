mod game;
mod window;
mod device;
mod swapchain;
mod pipeline;

use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;
use crate::{device::Device, swapchain::Swapchain};
use crate::window::Window;
use simple_logger::SimpleLogger;
use glfw::{self};
use glfw::{Action, Context, Key};


fn main() {
    println!("Hello World!");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    
    let mut window = Window::new(&mut glfw,"Revier:DEV BUILD #1",640,480);
    let mut device = Device::new(&window,&glfw);
    let mut swapchain = Swapchain::new(&device,window.getExtent());

    println!("{:?}",&swapchain.swapchain_image_views.unwrap().len());

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
