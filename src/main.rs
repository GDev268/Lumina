use std::{any::TypeId, rc::Rc};

use ash::vk::{self};

use revier_debug::logger::Logger;
use revier_render::camera::Camera;
use revier_geometry::{model::Model,shapes::{self}};
use revier_object::transform::Transform;
use revier_core::{device::Device,swapchain::Swapchain,window::Window};
use revier_scene::{query::Query,FrameInfo};
use revier_graphic::{renderer::PhysicalRenderer, shader::Shader};

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

    let mut renderer = PhysicalRenderer::new(&window, &device, None, Rc::clone(&shader));

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

    renderer.create_pipeline_layout(&device);
    renderer.create_pipeline(renderer.get_swapchain_renderpass(), &device);

    let mut camera = Camera::new();
    let mut view = Transform::default();
    let aspect = renderer.get_aspect_ratio();
    camera.set_perspective_projection(50.0_f32.to_radians(), aspect, 0.1, 10.0);
    camera.set_view_yxz(view.translation, view.rotation);


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

        if let Some(command_buffer) = renderer.begin_frame(&device, &window) {
            let frame_info: FrameInfo<'_> = FrameInfo {
                frame_time: 0.0,
                command_buffer,
                camera: &camera,
            };

            renderer.begin_swapchain_renderpass(command_buffer, &device);
            if let Some(transform) = query.query_mut::<Transform>(&cube) {
                transform.rotation.y =
                    (transform.rotation.y + 0.00055) % (std::f32::consts::PI * 2.0);
                transform.rotation.x =
                    (transform.rotation.x + 0.00055) % (std::f32::consts::PI * 2.0);
            }

            if let Some(transform) = query.query_mut::<Transform>(&cube2) {
                transform.rotation.y =
                    (transform.rotation.y - 0.00055) % (std::f32::consts::PI * 2.0);
                transform.rotation.x =
                    (transform.rotation.x - 0.00055) % (std::f32::consts::PI * 2.0);
            }


            renderer.render_game_objects(&device, &frame_info ,&mut query);

            renderer.end_swapchain_renderpass(command_buffer, &device);
        }

        renderer.end_frame(&device, &mut window);
    });
}