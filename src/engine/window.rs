use ash::vk::{self};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::{
    event_loop::EventLoop,
    window::{self, WindowBuilder},
};

pub struct Window {
    pub _window: window::Window,
    pub width: i32,
    pub height: i32,
    pub framebuffer_resized: bool,
    pub window_name: String,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>, title: &str, width: u32, height: u32) -> Self {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                f64::from(width),
                f64::from(height),
            ))
            .build(&event_loop)
            .unwrap();

        return Self {
            _window: window,
            width: width as i32,
            height: height as i32,
            framebuffer_resized: false,
            window_name: String::from(title),
        };
    }

    pub fn get_extent(&self) -> vk::Extent2D {
        return vk::Extent2D {
            width: self.width as u32,
            height: self.height as u32,
        };
    }

    pub fn was_window_resized(&self) -> bool {
        return self.framebuffer_resized;
    }

    pub fn reset_window_resized_flag(&mut self) {
        self.framebuffer_resized = false;
    }

    pub fn get_window(&self) -> &window::Window {
        return &self._window;
    }

    pub fn create_window_surface(
        &self,
        instance: &ash::Instance,
        entry: &ash::Entry,
    ) -> vk::SurfaceKHR {
        unsafe {
            return ash_window::create_surface(
                entry,
                instance,
                self._window.raw_display_handle(),
                self._window.raw_window_handle(),
                None,
            )
            .unwrap();
        }
    }

    pub fn framebuffer_resize_callback(window: &mut Window, width: i32, height: i32) {
        window.framebuffer_resized = true;
        window.width = width;
        window.height = height;
    }
}
