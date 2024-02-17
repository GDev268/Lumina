struct ClearColor([f32; 3]);
use ash::vk;
use lumina_render::renderer::Renderer;
use std::{rc::Rc, thread, time::Instant, f32::consts::E, sync::{Arc, RwLock}};

use lumina_bundle::{RendererBundle, ResourcesBundle};
use lumina_core::{device::Device, fps_manager::FPS, window::Window};
//use lumina_graphic::renderer::Renderer;
use lumina_input::{keyboard::Keyboard, mouse::Mouse};
use num_cpus;
use sdl2::Sdl;
use winit::event_loop::{EventLoop, EventLoopBuilder};

use crate::{query::Query, stage::Stage};

pub struct App {
    pub window: Window,
    pub device: Arc<Device>,
    pub renderer:Renderer,
    fps_manager: FPS,
    keyboard_pool: Keyboard,
    mouse_pool: Mouse,
    stage: Option<Stage>,
    start_tick: Instant,
    running: bool,
    focused: bool,
}

impl App {
    pub fn new(window:&Sdl) -> Self {
        let window = Window::new(window, "Lumina", 840, 680);
        let device = Arc::new(Device::new(&window));
        let renderer = Renderer::new(&window, &device,None);
        
        let mut fps_manager = FPS::new();
        fps_manager.set_max_fps(300);

        let renderer_bundle = Arc::new(RendererBundle {
            image_format: renderer.swapchain.get_swapchain_image_format(),
            depth_format: renderer.swapchain.get_swapchain_depth_format(),
            max_extent: vk::Extent2D {
                width: 800,
                height: 640,
            },
            render_pass: renderer.get_swapchain_renderpass(),
        });

        Self {
            window,
            device,
            renderer,
            fps_manager,
            keyboard_pool: Keyboard::new(),
            mouse_pool: Mouse::new(),
            stage: None,
            start_tick: Instant::now(),
            running: true,
            focused: true,
        }
    }

    pub fn switch_stage(&mut self, new_stage: Stage) {
        self.stage = Some(new_stage);
        //self.stage.as_mut().unwrap().create(Rc::clone(&self.device),self.renderer.get_aspect_ratio(),&self.window,&self.renderer_bundle);
    }

    pub fn update(&mut self) {
            /*self.stage
            .as_mut()
            .unwrap()
            .update(Arc::clone(&self.resources_bundle),self.fps_manager._fps as f32);*/
    }

    pub fn render(&mut self) {
        let command_buffer = self.renderer.begin_swapchain_command_buffer(&self.device, &self.window).unwrap();
        self.renderer.begin_frame(&self.device, command_buffer);
        self.renderer.begin_swapchain_renderpass(&self.device,command_buffer);

        println!("imagine if ninja got a low taper fade...");
        //self.stage.as_mut().unwrap().draw(Arc::clone(&self.resources_bundle), self.renderer.current_image_index,self.renderer.get_main_wait_semaphore(),self.renderer.get_current_command_buffer());

        self.renderer.end_swapchain_renderpass(command_buffer, &self.device);
        self.renderer.end_frame(&self.device, &mut self.window);
    }

    pub fn run(&mut self) {
        'running: loop {
            if self.running {
                break 'running;
            }

            if self.focused {
                self.update();
            }

            if self.start_tick.elapsed() < self.fps_manager.fps_limit {
                thread::sleep(self.fps_manager.fps_limit - self.start_tick.elapsed());
            }

            self.start_tick = Instant::now();
            self.fps_manager.update();
        }
    }

    pub fn create_event_loop() -> EventLoop<()> {
        let mut event_loop_builder = EventLoopBuilder::new();

        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::EventLoopBuilderExtWindows;
            event_loop_builder.with_any_thread(true);
        }

        #[cfg(target_os = "linux")]
        {
            //Need to find a way to check the support between wayland/x11
            //Wayland
            {
                use winit::platform::wayland::EventLoopBuilderExtWayland;
                event_loop_builder.with_any_thread(true);
            }
            //X11
            {
                use winit::platform::wayland::EventLoopBuilderExtX11;
                event_loop_builder.with_any_thread(true);
            }
        }

        return event_loop_builder.build();
    }

    pub fn get_device(&self) -> Arc<Device> {
        Arc::clone(&self.device)
    }

}
