use std::{
    any::TypeId,
    cell::RefCell,
    fs::File,
    io::Write,
    rc::Rc,
    thread,
    time::{Duration, Instant}, ops::Deref,
};

use ash::vk::{self, TRUE};
use glsl_parser::parser::Parser;
use rand::Rng;

use lumina_core::{device::Device, fps_manager::FPS, swapchain::Swapchain, window::Window, texture::Texture};

use lumina_data::{
    buffer::Buffer,
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, LayoutConfig, PoolConfig},
    descriptor_manager::{self, DescriptorManager},
};
use lumina_debug::logger::{Logger, SeverityLevel};
use lumina_geometry::{
    model::Model,
    shapes::{self},
};
use lumina_graphic::{renderer::Renderer, shader::Shader};
use lumina_input::{
    keyboard::{Keyboard, Keycode},
    mouse::{Mouse, MouseButton},
};
use lumina_object::{
    game_object::{self, GameObject},
    transform::Transform,
};
use lumina_render::camera::Camera;
use lumina_scene::{query::Query, FrameInfo, GlobalUBO};
//use lumina_pbr::material::Material;

use lazy_static::lazy_static;

use sdl2::{event::Event, image::LoadSurface};


lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

macro_rules! add {
    ($object:expr, $game_objects:expr) => {{
        let rc_object = Rc::new(RefCell::new($object));
        $game_objects.push(rc_object.clone());
    }};
}

macro_rules! trace {
    ($message:expr) => {
        let mut push = format!("{:?}", $message);
        if push.chars().nth(0).unwrap() == '"' && push.chars().nth(push.len() - 1).unwrap() == '"' {
            push.pop();
            push.remove(0);
        }
        LOGGER.push_message(push.as_str(), SeverityLevel::TRACE, None);
    };
}

macro_rules! info {
    ($message:expr) => {
        let mut push = format!("{:?}", $message);
        if push.chars().nth(0).unwrap() == '"' && push.chars().nth(push.len() - 1).unwrap() == '"' {
            push.pop();
            push.remove(0);
        }
        LOGGER.push_message(push.as_str(), SeverityLevel::INFO, None);
    };
}

macro_rules! warning {
    ($message:expr) => {
        let mut push = format!("{:?}", $message);
        if push.chars().nth(0).unwrap() == '"' && push.chars().nth(push.len() - 1).unwrap() == '"' {
            push.pop();
            push.remove(0);
        }
        LOGGER.push_message(push.as_str(), SeverityLevel::WARNING, None);
    };
}

macro_rules! error {
    ($message:expr) => {
        let mut push = format!("{:?}", $message);
        if push.chars().nth(0).unwrap() == '"' && push.chars().nth(push.len() - 1).unwrap() == '"' {
            push.pop();
            push.remove(0);
        }
        LOGGER.push_message(push.as_str(), SeverityLevel::ERROR, None);
    };
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
struct Material {
    ambient: [f32; 3],
    _padding1: [f32; 1],  // Padding to align 'diffuse' to a 16-byte boundary
    diffuse: [f32; 3],
    _padding2: [f32; 1],  // Padding to align 'specular' to a 16-byte boundary
    specular: [f32; 3],
    shininess: f32,
}

// No need to add explicit padding here, as Rust will handle it automatically
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
struct Light {
    position: [f32; 3],
    _padding1: [f32; 1],
    ambient: [f32; 3],
    _padding2: [f32; 1],
    diffuse: [f32; 3],
    _padding3: [f32; 1],
    specular: [f32; 3],
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
struct MaterialInfo {
    material: Material,
    view_pos: [f32; 3],
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
struct LightInfo {
    light: Light,
}


fn print_type_id<T: std::any::Any>(value: &T) {
    let type_id = TypeId::of::<T>();
    println!("Type ID: {:?}", type_id);
    println!("Type Size {:?}", std::mem::size_of::<T>());
}

fn main() {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        std::env::set_var("SDL_VIDEODRIVER", "wayland");
    }

    let sdl_context = sdl2::init().unwrap();

    let mut window = Window::new(&sdl_context, "Lumina Dev App", 800, 640);
    let device = Rc::new(Device::new(&window));

    let mut query = Query::new();

    let window_icon = sdl2::surface::Surface::from_file("icons/LuminaLogoMain.png").unwrap();

    window._window.set_icon(window_icon);
   
    let shader = Rc::new(RefCell::new(Shader::new(
        Rc::clone(&device),
        "shaders/default_shader.vert.spv",
        "shaders/default_shader.frag.spv",
    )));

    let mut renderer: Renderer = Renderer::new(&window, &device, Rc::clone(&shader), None);
    //renderer.activate_shader(&device, &shader);

    let mut keyboard_pool = Keyboard::new();

    let mut mouse_pool = Mouse::new();

    let mut game_objects: Vec<GameObject> = Vec::new();

    let mut cube = shapes::cube(&mut query, Rc::clone(&device));

    if let Some(transform) = query.query_mut::<Transform>(&cube) {
        transform.translation = glam::vec3(0.0, 0.0, 2.5);
        transform.scale = glam::vec3(1.0, 1.0, 1.0);
    }

    let mut pool_config = PoolConfig::new();
    pool_config.set_max_sets(3 * lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32);
    pool_config.add_pool_size(
        vk::DescriptorType::UNIFORM_BUFFER,
        3 * lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32,
    );


    let mut camera = Camera::new();

    let mut view = Transform::default();
    view.translation = glam::Vec3::ONE;
    view.rotation = glam::Vec3::ZERO;

    let aspect = renderer.get_aspect_ratio();
    camera.set_perspective_projection(50.0_f32.to_radians(), aspect, 0.1, 100.0);

    let mut time: f32 = 0.0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut fps = FPS::new();
    fps._fps = 300;
    let mut global_timer = Instant::now();
    let mut start_tick = Instant::now();

    fps.fps_limit = Duration::new(0, 1000000000u32 / fps._fps);
    let delta_time = 1.0 / fps._fps as f32;

    let light_pos = glam::vec3(3.0, -2.0, 100.0);

    'running: loop {
        start_tick = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    keyboard_pool.change_key_down(keycode as u32);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    keyboard_pool.change_key_up(keycode as u32);
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    mouse_pool.change_button(mouse_btn as u32);
                }
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    mouse_pool.change_motion(x, y, xrel, yrel);
                }
                _ => {}
            }
        }

        if keyboard_pool.get_key(Keycode::Escape) {
            break 'running;
        }

        if !keyboard_pool.get_key(Keycode::LShift) {
            if keyboard_pool.get_key(Keycode::Up) {
                view.translation.z += 10.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Down) {
                view.translation.z -= 10.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Right) {
                view.translation.x += 10.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Left) {
                view.translation.x -= 10.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Space) {
                view.translation.y -= 10.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::LCtrl) {
                view.translation.y += 10.0 * delta_time;
            }
        } else {
            if keyboard_pool.get_key(Keycode::Up) {
                view.translation.z += 50.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Down) {
                view.translation.z -= 50.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Right) {
                view.translation.x += 50.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Left) {
                view.translation.x -= 50.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Space) {
                view.translation.y -= 50.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::LCtrl) {
                view.translation.y += 50.0 * delta_time;
            }
        }

        if let Some(command_buffer) = renderer.begin_frame(&device, &window) {
            let frame_index = renderer.get_frame_index() as usize;

            let frame_info: FrameInfo<'_> = FrameInfo {
                frame_time: 0.0,
                command_buffer,
                camera: &camera,
                global_descriptor_set: shader.borrow_mut().descriptor_manager.get_descriptor_set(frame_index as u32),
            };

            renderer.begin_swapchain_renderpass(command_buffer, &device);

            let ubo: GlobalUBO = GlobalUBO {
                projection: (camera.get_projection() * camera.get_view()).to_cols_array_2d(),
                light_direction: (glam::vec3(1.0, 1.0, -1.0)).to_array(),
            };

            let material: MaterialInfo = MaterialInfo {
                material: Material {
                    ambient: [0.1, 0.1, 0.1],
                    diffuse: [0.0, 0.0, 0.0],
                    specular: [0.5, 0.5, 0.5],
                    shininess: 1.0,
                    _padding1: [0.0],
                    _padding2: [0.0]
                },
                view_pos: camera.get_position().to_array(),
            };

            let light: LightInfo = LightInfo {
                light: Light {
                    position: [3.0, -2.0, 3.0],
                    ambient: [0.1, 0.1, 0.1],
                    diffuse: [0.0, 0.0, 0.0],
                    specular: [0.25, 0.25, 0.25],
                    _padding1: [0.0],
                    _padding2: [0.0],
                    _padding3: [0.0]
                },
            };

            let texture = Texture::new(String::default(),Rc::clone(&device));

            shader.borrow_mut().descriptor_manager.change_buffer_value(
                "GlobalUBO".to_string(),
                frame_index as u32,
                &[ubo],
            );
            shader.borrow_mut().descriptor_manager.change_buffer_value(
                "MaterialInfo".to_string(),
                frame_index as u32,
                &[material],
            );
            shader.borrow_mut().descriptor_manager.change_buffer_value(
                "LightInfo".to_string(),
                frame_index as u32,
                &[light],
            );
            shader.borrow_mut().descriptor_manager.change_image_value(
                "sampler2D".to_string(),
                frame_index as u32,
                texture,
            );

            if let Some(transform) = query.query_mut::<Transform>(&cube) {
                transform.rotation += glam::vec3(100.0 * delta_time, 100.0 * delta_time, 100.0 * delta_time);
            }

            renderer.render_game_objects(&device, &frame_info, &mut query, Rc::clone(&shader));

            renderer.end_swapchain_renderpass(command_buffer, &device);
            camera.set_view_yxz(view.translation, view.rotation);
        }

        camera.set_view_yxz(view.translation, view.rotation);
        renderer.end_frame(&device, &mut window);

        print!("\rFPS: {:.2}", fps.frame_count / fps.frame_elapsed);
        let title = String::from("Lumina Dev App ")
            + format!("[FPS: {:.0}]", fps.frame_count / fps.frame_elapsed).as_str();
        window.get_window().set_title(title.as_str()).unwrap();
        if start_tick.elapsed() < fps.fps_limit {
            thread::sleep(fps.fps_limit - start_tick.elapsed());
        }
        time += 5.0 * delta_time;
        fps.update();
    }
}
