use std::{any::TypeId, fs::File, io::Write, rc::Rc, time::{Instant, Duration},thread};

use ash::vk::{self};
use rand::Rng;

use revier_core::{device::Device, swapchain::Swapchain, window::Window, fps_manager::FPS};

use revier_data::{
    buffer::Buffer,
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, PoolConfig},
};
use revier_debug::logger::Logger;
use revier_geometry::{
    model::Model,
    shapes::{self},
};
use revier_graphic::{renderer::PhysicalRenderer, shader::Shader};
use revier_input::{keyboard::{Keyboard, Keycode}, mouse::{Mouse, MouseButton}};
use revier_object::{game_object::GameObject, transform::Transform};
use revier_render::camera::Camera;
use revier_scene::{query::Query, FrameInfo, GlobalUBO};
use glsl_parser::parser::Parser;

use lazy_static::lazy_static;

use sdl2::{event::Event, image::LoadSurface};


macro_rules! add {
    ($object:expr, $game_objects:expr) => {{
        let rc_object = Rc::new(RefCell::new($object));
        $game_objects.push(rc_object.clone());
    }};
}

 
fn main() {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        std::env::set_var("SDL_VIDEODRIVER", "wayland");
    }

    //let event_loop = EventLoop::new();

    let sdl_context = sdl2::init().unwrap();

    let mut window = Window::new(&sdl_context, "Revier", 800, 640);
    let device = Device::new(&window);

    let _command_buffers: Vec<vk::CommandBuffer> = Vec::new();

    let mut query = Query::new();
    let mut parser = Parser::new();


    let window_icon = sdl2::surface::Surface::from_file("icons/2.png").unwrap();

    window._window.set_icon(window_icon);


     let shader = Rc::new(Shader::new(
        &device,
        "shaders/simple_shader.vert",
        "shaders/simple_shader.frag",
    ));


    let mut renderer = PhysicalRenderer::new(&window, &device, Rc::clone(&shader), None);

    let mut pool_config = PoolConfig::new();
    pool_config.set_max_sets(revier_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32);
    pool_config.add_pool_size(
        vk::DescriptorType::UNIFORM_BUFFER,
        revier_core::swapchain::MAX_FRAMES_IN_FLIGHT as u32,
    );

    let global_pool: DescriptorPool = pool_config.build(&device);

    let mut ubo_buffers: Vec<Buffer> = Vec::new();

    let mut keyboard_pool = Keyboard::new();

    let mut mouse_pool = Mouse::new();

    for i in 0..revier_core::swapchain::MAX_FRAMES_IN_FLIGHT {
        let mut buffer = Buffer::new(
            &device,
            std::mem::size_of::<GlobalUBO>() as u64,
            1,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
        );
        buffer.map(&device, None, None);

        ubo_buffers.push(buffer);
    }



    let global_set_layout = DescriptorSetLayout::build(
        &device,
        DescriptorSetLayout::add_binding(
            0,
            vk::DescriptorType::UNIFORM_BUFFER,
            vk::ShaderStageFlags::VERTEX,
            Some(1),
            None,
        ),
    );
    

    let mut global_descriptor_sets: Vec<vk::DescriptorSet> = Vec::new();

    for i in 0..revier_core::swapchain::MAX_FRAMES_IN_FLIGHT {
        let buffer_info = ubo_buffers[i].descriptor_info(None, None);
        let mut descriptor_writer = DescriptorWriter::new();
        descriptor_writer.write_buffer(0, buffer_info, &global_set_layout);
        let descriptor_set = descriptor_writer.build(&device, global_set_layout.get_descriptor_set_layout(), &global_pool);

        global_descriptor_sets.push(descriptor_set);
    }

    let mut cube = shapes::cube(&mut query, &device);

    if let Some(transform) = query.query_mut::<Transform>(&cube) {
        transform.translation = glam::vec3(0.0, 0.0, 2.5);
        transform.scale = glam::vec3(1.0, 1.0, 1.0);
    }


    let mut cube2 = shapes::cube(&mut query, &device);

    if let Some(transform) = query.query_mut::<Transform>(&cube2) {
        transform.translation = glam::vec3(1.0, 0.0, 5.0);
        transform.scale = glam::vec3(1.0, 1.0, 1.0);
    }



    renderer.create_pipeline_layout(&device,global_set_layout.get_descriptor_set_layout());
    renderer.create_pipeline(renderer.get_swapchain_renderpass(), &device);

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
    let benchmark_time = Instant::now();

    let mut light_pos = glam::vec3(0.0,3.0,4.0);
    
    fps.fps_limit =  Duration::new(0, 1000000000u32 / fps._fps);
    let delta_time = 1.0 / fps._fps as f32;
    println!("{:?}",fps.fps_limit);


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


        if let Some(command_buffer) = renderer.begin_frame(&device, &window) {
            let frame_index = renderer.get_frame_index() as usize;

            let frame_info: FrameInfo<'_> = FrameInfo {
                frame_time: 0.0,
                command_buffer,
                camera: &camera,
                global_descriptor_set: global_descriptor_sets[frame_index],
            };

            renderer.begin_swapchain_renderpass(command_buffer, &device);
   
          
            let ubo: GlobalUBO = GlobalUBO {
                projection: camera.get_projection() * camera.get_view(),
                light_direction: light_pos,
            };

            ubo_buffers[frame_index].write_to_buffer(&[ubo],None,None);
            ubo_buffers[frame_index].flush(None, None, &device);

            renderer.render_game_objects(&device, &frame_info, &mut query);

            renderer.end_swapchain_renderpass(command_buffer, &device);
            camera.set_view_yxz(view.translation, view.rotation);
        }
        
        renderer.end_frame(&device, &mut window);
       
        print!("\rFPS: {:.2}", fps.frame_count / fps.frame_elapsed);
        if start_tick.elapsed() < fps.fps_limit {
            thread::sleep(fps.fps_limit - start_tick.elapsed());
        }
        fps.update();
    }

}

