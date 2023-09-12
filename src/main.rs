use std::{any::TypeId, fs::File, io::Write, rc::Rc};

use ash::vk::{self};

use revier_core::{device::Device, swapchain::Swapchain, window::Window};
use revier_debug::logger::Logger;
use revier_geometry::{
    model::Model,
    shapes::{self},
};
use revier_graphic::{renderer::PhysicalRenderer, shader::Shader};
use revier_object::{game_object::GameObject, transform::Transform};
use revier_render::camera::Camera;
use revier_scene::{query::Query, FrameInfo};

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use lazy_static::lazy_static;

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
    let event_loop = EventLoop::new();

    let mut window = Window::new(&event_loop, "Hello Vulkan!", 800, 640);
    let device = Device::new(&window);

    let _command_buffers: Vec<vk::CommandBuffer> = Vec::new();

    let mut query = Query::new();

    let shader = Rc::new(Shader::new(
        &device,
        "shaders/simple_shader.vert.spv",
        "shaders/simple_shader.frag.spv",
    ));

    let mut renderer = PhysicalRenderer::new(&window, &device, Rc::clone(&shader), None);

    let mut game_objects: Vec<GameObject> = Vec::new();

    for i in 0..10 {
        for j in 0..75 {
            let mut cube = shapes::cube(&mut query, &device);
            if let Some(transform) = query.query_mut::<Transform>(&cube) {
                transform.translation =
                    glam::vec3(-29.0 + 1.0 * (j as f32), 3.0, 50.0 + 1.0 * (i as f32));
                transform.scale = glam::vec3(1.0, 1.0, 1.0);
            }

            game_objects.push(cube);
        }
    }

    println!("{:?}",query.entities.len());
    renderer.create_pipeline_layout(&device);
    renderer.create_pipeline(renderer.get_swapchain_renderpass(), &device);

    let mut camera = Camera::new();
    let mut view = Transform::default();
    let aspect = renderer.get_aspect_ratio();
    camera.set_perspective_projection(50.0_f32.to_radians(), aspect, 0.1, 100.0);
    camera.set_view_yxz(view.translation, view.rotation);

    let mut time: f32 = 0.0;


    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window._window.id() => control_flow.set_exit(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_size) => {
                    Window::framebuffer_resize_callback(
                        &mut window,
                        new_size.width,
                        new_size.height,
                    );
                }
                _ => (),
            },

            _ => (),
        }

        for i in 0..game_objects.len() {
            if let Some(transform) = query.query_mut::<Transform>(&game_objects[i]) {
                let wave = (std::f32::consts::PI / 30.0) * (transform.translation.x - (10.0 * time));

                transform.translation.y = 10.0 * wave.sin();
                transform.rotation.z =  0.5 * wave.sin();

            }
            
        }

        if let Some(command_buffer) = renderer.begin_frame(&device, &window) {
            /*let frame_info: FrameInfo<'_> = FrameInfo {
                frame_time: 0.0,
                command_buffer,
                camera: &camera,
            };*/

            renderer.begin_swapchain_renderpass(command_buffer, &device);

            //renderer.render_game_objects(&device, &frame_info, &mut query);

            renderer.end_swapchain_renderpass(command_buffer, &device);
            time += 0.005;
        }

        renderer.end_frame(&device, &mut window);
    });
}
