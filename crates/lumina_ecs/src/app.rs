/*struct ClearColor([f32; 3]);
use std::{rc::Rc, thread, time::Instant, f32::consts::E};

use lumina_core::{device::Device, fps_manager::FPS, window::Window};
//use lumina_graphic::renderer::Renderer;
use lumina_input::{keyboard::Keyboard, mouse::Mouse};
use lumina_render::system_renderer::SystemRenderer;
use num_cpus;
use sdl2::Sdl;
use winit::event_loop::{EventLoop, EventLoopBuilder};

use crate::{query::Query, stage::Stage};

pub struct App {
    window: Window,
    device: Device,
    renderer:SystemRenderer,
    fps_manager: FPS,
    keyboard_pool: Keyboard,
    mouse_pool: Mouse,
    stage: Option<Stage>,
    start_tick: Instant,
    running: bool,
    focused: bool,
}

impl App {
    pub fn new(sdl: &Sdl) -> Self {
        let event_loop = App::create_event_loop();
        let window = Window::new(&event_loop, "Lumina", 1920, 1080);
        let device = Device::new(&window);
        let renderer = SystemRenderer::new(&window, &device, None);
        let mut fps_manager = FPS::new();
        fps_manager.set_max_fps(300);

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
        self.stage.as_mut().unwrap().create();
    }

    pub fn update(&mut self) {
        self.stage
            .as_mut()
            .unwrap()
            .update(self.fps_manager._fps as f32);
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
}*/
