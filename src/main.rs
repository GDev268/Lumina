mod data;
mod graphics;
mod engine;
mod components;

use engine::{device::Device,window::Window};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[path = "testing/fill.rs"]
mod fill;


fn main() {
    let event_loop = EventLoop::new();

    let window = Window::new(&event_loop, "Hello Vulkan!", 800, 640);
    let _device = Device::new(&window);
    

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        let swapchain_support = _device.get_swapchain_support();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window._window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                let _ = &window._window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                fill::fill_window(&window._window);
                
            }
            _ => (),
            
        }
    });
}