use ash::vk::{self, Handle};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::{
    event_loop::EventLoop,
    window::{self, WindowBuilder}, dpi::PhysicalSize,
};

use sdl2::video;

pub struct Window {
    pub _window: winit::window::Window,
    pub width: u32,
    pub height: u32,
    pub framebuffer_resized: bool,
    pub window_name: String,
}

impl Window {
    pub fn new(event_loop:&EventLoop<()>, title: &str, width: u32, height: u32) -> Self {
        let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(event_loop).expect("Failed to create window");


        return Self {
            _window: window,
            width: width,
            height: height,
            framebuffer_resized: false,
            window_name: String::from(title),
        };
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        return self._window.inner_size();
    }

    pub fn was_window_resized(&self) -> bool {
        return self.framebuffer_resized;
    }

    pub fn reset_window_resized_flag(&mut self) {
        self.framebuffer_resized = false;
    }

    pub fn get_window(&self) -> &winit::window::Window {
        return &self._window;
    }


    pub fn resize(&mut self, new_size:PhysicalSize<u32>) {
        self.framebuffer_resized = true;
        self.width = new_size.width;
        self.height = new_size.height;
    }
}
