use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::cell::RefCell;
use winit::{
    event_loop::EventLoop,
    window::{self, WindowBuilder},
};
use ash::vk;
pub struct Window {
    window: window::Window,
    width: i16,
    height: i16,
    framebuffer_resized: bool,
    window_name: String,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>, title: &str, width: i16, height: i16) -> Self {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                f64::from(width),
                f64::from(height),
            ))
            .build(event_loop)
            .unwrap();

        return Self {
            window: window,
            width: width,
            height: height,
            framebuffer_resized: false,
            window_name: String::from(title),
        };
    }

    pub fn getExtent(&self) -> vk::Extent2D {
        return vk::Extent2D {
            width: self.width as u32,
            height: self.height as u32,
        };
    }

    pub fn resetWindowResizedFlag(&self) -> bool {
        return self.framebuffer_resized;
    }

    pub fn getWindow(&self) -> &window::Window {
        return &self.window;
    }

    pub fn createWindowSurface(
        &self,
        instance: &ash::Instance,
        entry: &ash::Entry,
        window: window::Window,
    ) -> vk::SurfaceKHR {
        unsafe {
            return ash_window::create_surface(
                entry,
                instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
            .unwrap();
        }
    }

    fn framebufferResizeCallback(window: WindowBuilder, width: i16, height: i16) {}
}
