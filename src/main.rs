use std::{any::TypeId, fs::File, io::Write, rc::Rc, time::{Instant, Duration},thread};

use ash::vk::{self};
use rand::Rng;

use lumina_core::{device::Device, swapchain::Swapchain, window::Window, fps_manager::FPS};

use lumina_data::{
    buffer::Buffer,
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, PoolConfig},
};
use lumina_debug::logger::{Logger,SeverityLevel};
use lumina_geometry::{
    model::Model,
    shapes::{self},
};
use lumina_graphic::{renderer::Renderer, shader::Shader};
use lumina_input::{keyboard::{Keyboard, Keycode}, mouse::{Mouse, MouseButton}};
use lumina_object::{game_object::GameObject, transform::Transform};
use lumina_render::camera::Camera;
use lumina_scene::{query::Query, FrameInfo, GlobalUBO};
use glsl_parser::parser::Parser;

use lazy_static::lazy_static;

use sdl2::{event::Event, image::LoadSurface};


lazy_static!(
    static ref LOGGER:Logger = Logger::new();
);

macro_rules! add {
    ($object:expr, $game_objects:expr) => {{
        let rc_object = Rc::new(RefCell::new($object));
        $game_objects.push(rc_object.clone());
    }};
}

macro_rules! trace {
    ($message:expr) => {
        let mut push = format!("{:?}",$message);
        if push.chars().nth(0).unwrap() == '"' && push.chars().nth(push.len() - 1).unwrap() == '"'{
            push.pop();
            push.remove(0);
        }
        LOGGER.push_message(push.as_str(),SeverityLevel::TRACE,None);
    };
}

macro_rules! info {
    ($message:expr) => {
        let mut push = format!("{:?}",$message);
        if push.chars().nth(0).unwrap() == '"' && push.chars().nth(push.len() - 1).unwrap() == '"'{
            push.pop();
            push.remove(0);
        }
        LOGGER.push_message(push.as_str(),SeverityLevel::INFO,None);
    };
}

macro_rules! warning {
    ($message:expr) => {
        let mut push = format!("{:?}",$message);
        if push.chars().nth(0).unwrap() == '"' && push.chars().nth(push.len() - 1).unwrap() == '"'{
            push.pop();
            push.remove(0);
        }
        LOGGER.push_message(push.as_str(),SeverityLevel::WARNING,None);
    };
}

macro_rules! error {
    ($message:expr) => {
        let mut push = format!("{:?}",$message);
        if push.chars().nth(0).unwrap() == '"' && push.chars().nth(push.len() - 1).unwrap() == '"'{
            push.pop();
            push.remove(0);
        }
        LOGGER.push_message(push.as_str(),SeverityLevel::ERROR,None);
    };
}


fn main() {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        std::env::set_var("SDL_VIDEODRIVER", "wayland");
    }

    let sdl_context = sdl2::init().unwrap();

    let mut window = Window::new(&sdl_context, "lumina", 800, 640);
    let device = Device::new(&window);

    let _command_buffers: Vec<vk::CommandBuffer> = Vec::new();

    let mut query = Query::new();

    let window_icon = sdl2::surface::Surface::from_file("icons/LuminaLogoMain.png").unwrap();

    let mut pool_config = PoolConfig::new();
    pool_config.set_max_sets(2 * lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32);
    pool_config.add_pool_size(
        vk::DescriptorType::UNIFORM_BUFFER,
        2 * lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32,
    );

    window._window.set_icon(window_icon);

    let shader = Shader::new(
        &device,
        "shaders/default_shader.vert",
        "shaders/default_shader.frag",
         pool_config
    );


    let mut renderer = Renderer::new(&window, &device);

    //renderer.activate_shader(&device, &shader);

    let ubo_buffers: Vec<Buffer> = Vec::new();

    let mut keyboard_pool = Keyboard::new();

    let mut mouse_pool = Mouse::new();

    let mut game_objects: Vec<GameObject> = Vec::new();
    

    let mut pool_config = PoolConfig::new();
        pool_config.set_max_sets(2 * lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32);
        pool_config.add_pool_size(
        vk::DescriptorType::UNIFORM_BUFFER,
        2 * lumina_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32,
    );

    let shader = Shader::new(
        &device,
        "shaders/default_shader.vert",
        "shaders/default_shader.frag",
        pool_config
    );

    let mut cube = shapes::cube(&mut query, &device);

    if let Some(transform) = query.query_mut::<Transform>(&cube) {
        transform.translation = glam::vec3(0.0, 0.0, 2.5);
        transform.scale = glam::vec3(1.0, 1.0, 1.0);
    }

    query.push(&cube,shader);
 
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

    let mut light_pos = glam::vec3(1.0, 2.0, 2.0);
    
    fps.fps_limit =  Duration::new(0, 1000000000u32 / fps._fps);
    let delta_time = 1.0 / fps._fps as f32;

    'running: loop {

        start_tick = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode:Some(keycode), ..} => {
                    keyboard_pool.change_key_down(keycode as u32); 
                },
                Event::KeyUp { keycode:Some(keycode), .. } => {
                    keyboard_pool.change_key_up(keycode as u32); 
                }
                Event::MouseButtonDown {mouse_btn, ..} => {
                    mouse_pool.change_button(mouse_btn as u32);
                },
                Event::MouseMotion{x, y, xrel, yrel,.. } => {
                    mouse_pool.change_motion(x, y, xrel, yrel);
                }
                _ => {}
            }

        }

        if keyboard_pool.get_key(Keycode::Escape){
            break 'running;
        }
        
        if !keyboard_pool.get_key(Keycode::LShift){
            if keyboard_pool.get_key(Keycode::Up){
                view.translation.z += 10.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Down){
                view.translation.z -= 10.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Right){
                view.translation.x += 10.0 * delta_time;
            } 
            if keyboard_pool.get_key(Keycode::Left){
                view.translation.x -= 10.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Space){
                view.translation.y -= 10.0 * delta_time;
            }       
            if keyboard_pool.get_key(Keycode::LCtrl){
                view.translation.y += 10.0 * delta_time;
            }
        }else{
            if keyboard_pool.get_key(Keycode::Up){
                view.translation.z += 50.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Down){
                view.translation.z -= 50.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Right){
                view.translation.x += 50.0 * delta_time;
            } 
            if keyboard_pool.get_key(Keycode::Left){
                view.translation.x -= 50.0 * delta_time;
            }
            if keyboard_pool.get_key(Keycode::Space){
                view.translation.y -= 50.0 * delta_time;
            }       
            if keyboard_pool.get_key(Keycode::LCtrl){
                view.translation.y += 50.0 * delta_time;
            }
        }

        renderer.begin_frame(&device, &window);
       


        let game_transform = query.query_mut::<Transform>(&cube).unwrap(); 
        /*let wave = (std::f32::consts::PI / 30.0) * (game_transform.translation.x - (10.0 * time));
        game_transform.translation.y = 10.0 * wave.cos();*/

        let new_mat4 = game_transform.get_mat4();
        drop(game_transform);
        let new_normal = query.query_mut::<Transform>(&cube).unwrap().get_normal_matrix(); 

        if let Some(shader) = query.query_mut::<Shader>(&cube) {
            shader.change_uniform_mat4("GlobalUBO.projectionViewMatrix", camera.get_projection() * camera.get_view()).unwrap();
            shader.change_uniform_vec3("GlobalUBO.directionToLight", light_pos).unwrap();
            shader.change_uniform_mat4("Push.modelMatrix",new_mat4).unwrap();
            shader.change_uniform_mat4("Push.normalMatrix", new_normal).unwrap();
        }

        renderer.render_object(&device, &mut query,&cube);
        

 
        camera.set_view_yxz(view.translation, view.rotation);
        renderer.end_frame(&device, &mut window);
       
        print!("\rFPS: {:.2}", fps.frame_count / fps.frame_elapsed);
        if start_tick.elapsed() < fps.fps_limit {
            thread::sleep(fps.fps_limit - start_tick.elapsed());
        }
        time += 5.0 * delta_time;
        fps.update();
    }

}

