extern crate ash;
extern crate winit;

mod game;
mod window;
mod device;
mod swapchain;

use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;
use crate::{device::Device, swapchain::Swapchain};
use crate::window::Window;
use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};


#[path = "testing/fill.rs"]
mod fill;
fn main() {
    println!("Hello World!");
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop,"Revier:DEV BUILD #1",640,480);
    let mut device = Device::new(&window);


        event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window._window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                &window._window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                fill::fill_window(&window._window);
            }
            _ => (),
            
        }
    });
    
}


fn asdasda(device: &mut Device){
    asdas(device);
}

fn asdas(device:&mut Device){
    println!("{:?}",device.game_version);
}