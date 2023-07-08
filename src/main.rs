extern crate ash;
extern crate winit;

mod game;
mod window;
mod device;

use std::borrow::Borrow;

use crate::device::Device;
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
    let window = Window::new(&event_loop,"ayes",640,480);
    let device = Device::new(window);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == device.window.as_ref().unwrap()._window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                &device.window.as_ref().unwrap()._window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                fill::fill_window(&device.window.as_ref().unwrap()._window);
            }
            _ => (),
        }
    });
}
