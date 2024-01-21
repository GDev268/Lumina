use ash::vk::{self, Handle};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::{
    event_loop::EventLoop,
    window::{self, WindowBuilder},
};

use sdl2::video;

pub struct Window {
    pub _window: window::Window,
    pub width: u32,
    pub height: u32,
    pub framebuffer_resized: bool,
    pub window_name: String,
}

impl Window {
    pub fn new(_window:window::Window, title: &str, width: u32, height: u32) -> Self {
        _window.set_title(title);
        _window.set_inner_size(winit::dpi::PhysicalSize::new(width, height));

        return Self {
            _window,
            width: width,
            height: height,
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

    pub fn get_window(&mut self) -> &mut window::Window {
        return &mut self._window;
    }

    pub fn create_window_surface(
        &self,
        instance: &ash::Instance,
        entry: &ash::Entry,
    ) -> vk::SurfaceKHR {

        return unsafe { ash_window::create_surface(entry, instance, self._window.raw_display_handle(), self._window.raw_window_handle(), None).unwrap() };
    }

    pub fn framebuffer_resize_callback(window: &mut Window, width: u32, height: u32) {
        window.framebuffer_resized = true;
        window.width = width;
        window.height = height;
    }
}
