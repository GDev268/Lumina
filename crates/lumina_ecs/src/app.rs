struct ClearColor([f32; 3]);
use std::{rc::Rc, thread, time::Instant};

use lumina_core::{device::Device, fps_manager::FPS, window::Window};
//use lumina_graphic::renderer::Renderer;
use lumina_input::{keyboard::Keyboard, mouse::Mouse};
use num_cpus;
use sdl2::Sdl;

use crate::{query::Query, stage::Stage};

struct App {
    window: Window,
    device: Device,
    //renderer: Renderer,
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
        let window = Window::new(sdl, "Lumina", 1920, 1080);
        let device = Device::new(&window);
        //let renderer = Renderer::new(&window, &device, None);
        let mut fps_manager = FPS::new();
        fps_manager.set_max_fps(300);

        Self {
            window,
            device,
            //renderer,
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

    /*pub fn update(&mut self) {
        let num_cpus = num_cpus::get().max(1);

        let chunk_size = self.query.entities.lock().as_ref().borrow().unwrap().len() / num_cpus;


        let entity_clone = Arc::clone(&self.entities);
        let start = i * chunk_size;
        let end = if i == num_cpus - 1 {
            self.entities.lock().as_ref().borrow().unwrap().len()
        } else {
            (i + 1) * chunk_size
        };
        let mut cloned_entities = Arc::clone(&self.entities);

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let entity_clone = Arc::clone(&self.entities);
                let start = i * chunk_size;
                let end = if i == num_cpus - 1 {
                    self.entities.lock().as_ref().borrow().unwrap().len()
                } else {
                    (i + 1) * chunk_size
                };
                let mut cloned_entities = Arc::clone(&self.entities);

                thread::spawn(move || {
                    for entity in cloned_entities
                        .borrow_mut()
                        .lock()
                        .unwrap()
                        .values()
                        .skip(start)
                        .take(end - start)
                    {}
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }*/
}
