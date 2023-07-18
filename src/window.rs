use ash::vk;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::{cell::RefCell, sync::mpsc::Receiver};
use glfw::{Action, Context, Key,Glfw, WindowEvent};

pub struct Window {
    pub _window: glfw::Window,
    pub events:Receiver<(f64, WindowEvent)>,
    pub width: u32,
    pub height: u32,
    pub framebuffer_resized: bool,
    pub window_name: String,
}

impl Window {
    pub fn new(glfw: &mut Glfw, title: &str, width: u32, height: u32) -> Self {
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);


        return Self {
            _window: window,
            events:events,
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

    pub fn getWindow(&self) -> &glfw::Window {
        return &self._window;
    }

    pub fn createWindowSurface(
        &self,
        instance: &ash::Instance,
        entry: &ash::Entry,
    ) -> vk::SurfaceKHR {
        unsafe {
            let mut surface: std::mem::MaybeUninit<vk::SurfaceKHR> = std::mem::MaybeUninit::uninit();
            self._window.create_window_surface(instance.handle(), std::ptr::null(), surface.as_mut_ptr());
            return surface.assume_init_read();
        }
    }

    fn framebufferResizeCallback(window: &Glfw, width: i16, height: i16) {}
}
