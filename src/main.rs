use std::{any::TypeId, fs::File, io::Write, rc::Rc};

use ash::vk::{self};

use revier_core::{device::Device, swapchain::Swapchain, window::Window};
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
use revier_input::keyboard::{Keyboard, Keycode};
use revier_object::{game_object::GameObject, transform::Transform};
use revier_render::camera::Camera;
use revier_scene::{query::Query, FrameInfo, GlobalUBO};


use lazy_static::lazy_static;

use sdl2::event::Event;

// Create a lazy-static instance of your global struct
lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

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

    let shader = Rc::new(Shader::new(
        &device,
        "shaders/simple_shader.vert.spv",
        "shaders/simple_shader.frag.spv",
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

    let mut game_objects: Vec<GameObject> = Vec::new();

    for i in 0..10 {
        for j in 0..75 {
            let cube = shapes::cube(&mut query, &device);
            if let Some(transform) = query.query_mut::<Transform>(&cube) {
                transform.translation =
                    glam::vec3(-29.0 + 1.0 * (j as f32), 3.0, 50.0 + 1.0 * (i as f32));
                transform.scale = glam::vec3(1.0, 1.0, 1.0);
            }

            game_objects.push(cube);
        }
    }

    renderer.create_pipeline_layout(&device,global_set_layout.get_descriptor_set_layout());
    renderer.create_pipeline(renderer.get_swapchain_renderpass(), &device);

    let mut camera = Camera::new();

    let mut view = Transform::default();
    view.translation = glam::Vec3::ONE;
    view.rotation = glam::Vec3::ZERO;

    let aspect = renderer.get_aspect_ratio();
    camera.set_perspective_projection(50.0_f32.to_radians(), aspect, 0.1, 100.0);
    camera.set_view_yxz(view.translation, view.rotation);

    let mut time: f32 = 0.0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        keyboard_pool.reset_keys(); 

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode, .. } => {
                    keyboard_pool.change_key(keycode.unwrap() as u32); 
                },
                _ => {}
            }

        }
        if keyboard_pool.get_key(Keycode::Escape){
            break 'running;
        }



        for i in 0..game_objects.len() {
            if let Some(transform) = query.query_mut::<Transform>(&game_objects[i]) {
                let wave =
                    (std::f32::consts::PI / 30.0) * (transform.translation.x - (10.0 * time));

                transform.translation.y = 10.0 * wave.sin();
                transform.rotation.z = 0.5 * wave.sin();
            }
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
                light_direction: glam::vec3(1.0, -2.0, -1.0),
            };

            ubo_buffers[frame_index].write_to_buffer(&[ubo],None,None);
            ubo_buffers[frame_index].flush(None, None, &device);

            renderer.render_game_objects(&device, &frame_info, &mut query);

            renderer.end_swapchain_renderpass(command_buffer, &device);
            time += 0.005;
        }

        for (key,value) in &keyboard_pool.keys{
            if *value{
                println!("Just pressed: {:?}",keyboard_pool.from_u32(&key).unwrap());
            }
        }
        renderer.end_frame(&device, &mut window);
    }

}
