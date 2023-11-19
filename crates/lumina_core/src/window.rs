use ash::vk::{self, Handle};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::{
    event_loop::EventLoop,
    window::{self, WindowBuilder},
};

use sdl2::video;

pub struct Window {
    pub _window: video::Window,
    pub width: u32,
    pub height: u32,
    pub framebuffer_resized: bool,
    pub window_name: String,
}

impl Window {
    pub fn new(sdl:&sdl2::Sdl, title: &str, width: u32, height: u32) -> Self {
        let window_subsystem = sdl.video().unwrap();

        let window = window_subsystem.window(title, width, height).resizable().vulkan().build().expect("Failed to create sdl2 window!");

        return Self {
            _window: window,
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

    pub fn get_window(&mut self) -> &mut video::Window {
        return &mut self._window;
    }

    pub fn create_window_surface(
        &self,
        instance: &ash::Instance,
    ) -> vk::SurfaceKHR {
        let raw_instance = instance.handle().as_raw() as usize;

        let raw_surface = self._window.vulkan_create_surface(raw_instance).expect("Failed to create vulkan surface");
        
        return vk::SurfaceKHR::from_raw(raw_surface);
    }

    pub fn framebuffer_resize_callback(window: &mut Window, width: u32, height: u32) {
        window.framebuffer_resized = true;
        window.width = width;
        window.height = height;
    }
}
