struct ClearColor([f32; 3]);
use ash::vk;
use lumina_files::{loader::Loader, saver::Saver};
use lumina_pbr::light::Light;
use lumina_render::renderer::Renderer;
use rand::Rng;
use serde_json::Value;
use std::{
    f32::consts::E,
    rc::Rc,
    sync::{Arc, RwLock},
    thread,
    time::Instant,
};

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
    pub renderer: Arc<RwLock<Renderer>>,
    fps_manager: FPS,
    keyboard_pool: Keyboard,
    mouse_pool: Mouse,
    stage: Option<Stage>,
    start_tick: Instant,
    running: bool,
    focused: bool,
}

impl App {
    pub fn new(window: &Sdl) -> Self {
        let window = Window::new(window, "Lumina", 1280, 720);
        let device = Arc::new(Device::new(&window));
        let renderer = Arc::new(RwLock::new(Renderer::new(&window, &device, None)));

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
        //self.stage.as_mut().unwrap().create(Rc::clone(&self.device),self.renderer.get_aspect_ratio(),&self.window,&self.renderer_bundle);
    }

    pub fn update(&mut self) {
        /*self.stage
        .as_mut()
        .unwrap()
        .update(Arc::clone(&self.resources_bundle),self.fps_manager._fps as f32);*/
    }

    pub fn load_file(&mut self, file_path: &str) {
        let mut loader = Loader::new();

        loader.load_file(file_path.to_string());

        let file_content = loader
            .directories
            .get("gameData")
            .unwrap()
            .files
            .iter()
            .find(|file| "scene.json" == file.file_name)
            .unwrap()
            .file_content
            .clone();

        let json_string = String::from_utf8(file_content).unwrap();
        let json: Value = serde_json::from_str(&json_string).unwrap();
        println!("JSON Content: {:?}", serde_json::to_string_pretty(&json));

        //self.stage = Stage::new("");
    }

    pub fn save_scene(&mut self) {
        let mut saver = Saver::new();

        self.stage = Some(Stage::new("test"));

        let mut rng = rand::thread_rng();

        for i in 0..10 {
            let light = self.stage.as_mut().unwrap().manager.spawn();

            let mut light_component = Light::new();
            light_component.change_color(glam::vec3(
                rng.gen_range(0, 20) as f32,
                rng.gen_range(0, 20) as f32,
                rng.gen_range(0, 20) as f32,
            ));
            light_component.change_intensity(rng.gen_range(0, 20) as f32);
            light_component.change_light_type(rng.gen_range(0, 3));
            light_component.change_range(rng.gen_range(0, 20) as f32);
            light_component.change_spot_size(rng.gen_range(0, 20) as f32);

            self.stage
                .as_mut()
                .unwrap()
                .manager
                .push(&light, light_component);
        }

        saver.modify_project_name(&self.stage.as_ref().unwrap().name);

        let lights = self.stage.as_ref().unwrap().get_light_json();

        saver.modify_array_value("lights", lights);

        saver.save_data();
    }

    /*pub fn render(&mut self) {
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
    }*/

    pub fn get_device(&self) -> Arc<Device> {
        Arc::clone(&self.device)
    }
}


impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            self.renderer.write().unwrap().cleanup(&self.device);
            self.device.device().device_wait_idle().unwrap();
            self.device.cleanup();
        }
    }
}