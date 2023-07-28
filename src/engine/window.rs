use ash::vk::{self, Handle};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::{cell::RefCell, sync::mpsc::Receiver};
use glfw::{Action, Context, Key,Glfw, WindowEvent};

pub struct Window {
    pub _window: glfw::Window,
    pub events:Receiver<(f64, WindowEvent)>,
    pub width: i32,
    pub height: i32,
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
            width: width as i32,
            height: height as i32,
            framebuffer_resized: false,
            window_name: String::from(title),
        };
    }

    pub fn should_close(&self) -> bool{
        return self._window.should_close();
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

    pub fn get_window(&self) -> &glfw::Window {
        return &self._window;
    }

    pub fn create_window_surface(
        &self,
        instance: &ash::Instance,
        entry: &ash::Entry,
    ) -> vk::SurfaceKHR {
            let mut surface: std::mem::MaybeUninit<vk::SurfaceKHR> = std::mem::MaybeUninit::uninit();

            if self._window.create_window_surface(instance.handle(), std::ptr::null(), surface.as_mut_ptr())
                != vk::Result::SUCCESS
            {
                panic!("Failed to create GLFW window surface.");
            }

            unsafe{
                return surface.assume_init();
            }
    }

    pub fn framebuffer_resize_callback(window:&mut Window,width: i32, height: i32) {
        window.framebuffer_resized = true;
        window.width = width;
        window.height = height;
    }
}
