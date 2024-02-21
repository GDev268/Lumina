use std::{
    any::TypeId,
    cell::RefCell,
    fs::File,
    io::Write,
    ops::Deref,
    rc::Rc,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use ash::vk::{self, TRUE};
use cgmath::num_traits::float::FloatCore;
use egui::{ColorImage, ImageData, Key};
use glsl_parser::parser::Parser;
use image::{DynamicImage, ImageBuffer, Luma, Rgba};
use lumina_ecs::{app::App, query::Query, stage::Stage};
use lumina_pbr::light::{DirectionalLight, Light};
use rand::Rng;

use lumina_core::{
    device::Device, fps_manager::FPS, swapchain::Swapchain, texture::Texture, window::Window,
};

use lumina_data::{
    buffer::Buffer,
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, LayoutConfig, PoolConfig},
    descriptor_manager::{self, DescriptorManager},
};
use lumina_debug::logger::{Logger, SeverityLevel};
use lumina_geometry::shapes;
use lumina_graphic::shader::{PushConstantData, Shader};
use lumina_input::{
    keyboard::{Keyboard, Keycode},
    mouse::{Mouse, MouseButton},
};
use lumina_object::{
    game_object::{self, GameObject},
    transform::Transform,
};
use lumina_render::{
    camera::{Camera, CameraDirection},
    model::Model,
    renderer::Renderer,
};
use lumina_scene::{FrameInfo, GlobalUBO};
//use lumina_pbr::material::Material;

use lazy_static::lazy_static;

use sdl2::{event::Event, Sdl};

lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
struct Material {
    ambient: [f32; 3],
    _padding1: [f32; 1],
    diffuse: [f32; 3],
    _padding2: [f32; 1],
    specular: [f32; 3],
    shininess: f32,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct RawLight {
    pub position: [f32; 3],
    pub _padding1: u32,

    pub color: [f32; 3],
    pub _padding2: u32,

    pub rotation: [f32; 3],
    //pub _padding3: u32,
    pub intensity: f32,

    pub spot_size: f32,

    pub linear: f32,
    pub quadratic: f32,

    pub light_type: u32,
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
    light: RawLight,
}

fn print_type_id<T: std::any::Any>(value: &T) {
    let type_id = TypeId::of::<T>();
    println!("Type ID: {:?}", type_id);
    println!("Type Size {:?}", std::mem::size_of::<T>());
}

fn main() {
    let sdl = sdl2::init().unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let mut video = sdl.video().unwrap();

    let mut app = App::new(&sdl);

    sdl.mouse().set_relative_mouse_mode(true);

    let mut query = Query::new();

    let mut keyboard_pool = Keyboard::new();

    let mut mouse_pool = Mouse::new();

    let mut game_objects: Vec<GameObject> = Vec::new();

    let mut platform = egui_sdl2_platform::Platform::new(app.window._window.size()).unwrap();

    let cube_positions: [glam::Vec3; 10] = [
        glam::Vec3::new(0.0, 0.0, 0.0),
        glam::Vec3::new(2.0, 5.0, -15.0),
        glam::Vec3::new(-1.5, -2.2, -2.5),
        glam::Vec3::new(-3.8, -2.0, -12.3),
        glam::Vec3::new(2.4, -0.4, -3.5),
        glam::Vec3::new(-1.7, 3.0, -7.5),
        glam::Vec3::new(1.3, -2.0, -2.5),
        glam::Vec3::new(1.5, 2.0, -2.5),
        glam::Vec3::new(1.5, 0.2, -1.5),
        glam::Vec3::new(-1.3, 1.0, -1.5),
    ];

    let mut stage = Stage::new("fasf");

    for i in 0..1 {
        let model = Model::new_from_model(app.get_device(), "models/Sponza.gltf");

        let cube = stage.manager.spawn();

        stage.manager.push(&cube, model);

        if let Some(transform) = stage
            .manager
            .query_entity(&cube)
            .unwrap()
            .write()
            .unwrap()
            .get_mut_component::<Transform>()
        {
            transform.translation = cube_positions[i];
            transform.scale = glam::vec3(-0.012, -0.012, -0.012);
            let angle = 20.0 * i as f32;
            transform.rotation = glam::vec3(angle, angle, angle);
        }

        if let Some(model) = stage
            .manager
            .query_entity(&cube)
            .unwrap()
            .write()
            .unwrap()
            .get_mut_component::<Model>()
        {
            model.shader.create_pipeline_layout(true);
            model
                .shader
                .create_pipeline(app.renderer.read().unwrap().get_swapchain_renderpass());
            model
                .shader
                .descriptor_manager
                .change_image_size("colorMap", 1024, 1024);
            model
                .shader
                .descriptor_manager
                .change_image_size("normalMap", 1024, 1024);
            model
                .shader
                .descriptor_manager
                .change_image_size("specularMap", 1024, 1024);
            model
                .shader
                .descriptor_manager
                .change_buffer_count("LightInfo", 2);

            model.shader.descriptor_manager.update_we();

            //model.shader.renovate_pipeline(app.renderer.read().unwrap().get_swapchain_renderpass());
        }
        game_objects.push(cube);
    }

    let mut model = shapes::model_cube(app.get_device());

    let skybox = stage.manager.spawn();

    model.shader = Shader::new(
        app.get_device(),
        "shaders/skybox_shader.vert",
        "shaders/skybox_shader.frag",
    );

    stage.manager.push(&skybox, model);

    if let Some(transform) = stage
        .manager
        .query_entity(&skybox)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Transform>()
    {
        transform.translation = glam::Vec3::ZERO;
        transform.scale = glam::vec3(1000.0, 1000.0, 1000.0);
    }

    if let Some(model) = stage
        .manager
        .query_entity(&skybox)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Model>()
    {
        model.shader.create_pipeline_layout(true);
        model
            .shader
            .create_pipeline(app.renderer.read().unwrap().get_swapchain_renderpass());
        model
            .shader
            .descriptor_manager
            .change_image_size("skybox", 256, 256);
        model.shader.descriptor_manager.update_we();

        //model.shader.renovate_pipeline(app.renderer.read().unwrap().get_swapchain_renderpass());
    }

    let mut camera = Camera::new(app.renderer.read().unwrap().get_aspect_ratio(), false);

    camera.speed = 10.0;

    let texture = Texture::new("models/brickwall.jpg".to_owned());
    let normal = Texture::new("models/brickwall_normal.jpg".to_owned());
    let specular = Texture::new("models/brickwall_specular.jpg".to_owned());

    let material: MaterialInfo = MaterialInfo {
        material: Material {
            ambient: [0.1, 0.1, 0.1],
            diffuse: [0.0, 0.0, 0.0],
            specular: [0.1, 0.1, 0.1],
            shininess: 1.0,
            _padding1: [0.0],
            _padding2: [0.0],
        },
        view_pos: camera.get_position().to_array(),
    };

    let raw_light: LightInfo = LightInfo {
        light: RawLight {
            position: [0.2, 0.0, 3.0],
            rotation: [-0.6, 0.0, -0.9],
            color: [0.0, 1.0, 1.0],
            intensity: 20.0,
            spot_size: 12.5,
            linear: 0.7,
            quadratic: 1.8,
            light_type: 1,
            _padding1: 0,
            _padding2: 0,
        },
    };

    let raw_light_2: LightInfo = LightInfo {
        light: RawLight {
            position: [0.0, -50.0, 3.0],
            rotation: [-0.0, 0.0, -0.0],
            color: [1.0, 1.0, 1.0],
            intensity: 5.0,
            spot_size: 12.0,
            linear: 0.7,
            quadratic: 1.8,
            light_type: 2,
            _padding1: 0,
            _padding2: 0,
        },
    };

    let light = stage.manager.spawn();

    if let Some(transform) = stage
        .manager
        .query_entity(&light)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Transform>()
    {
        transform.translation = glam::vec3(1.5, 0.0, 3.0);
        transform.rotation = glam::vec3(-0.6, 0.0, -0.9);
    }

    let mut light_component = DirectionalLight::new();
    light_component.change_color(glam::vec3(1.0, 1.0, 1.0));
    light_component.change_intensity(20.0);

    stage.manager.push(&light, light_component);

    let light_2 = stage.manager.spawn();

    if let Some(transform) = stage
        .manager
        .query_entity(&light_2)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Transform>()
    {
        transform.translation = glam::vec3(0.0, -500.0, 0.0);
        transform.rotation = glam::vec3(-0.6, 0.0, -0.9);
    }

    let mut light_component = DirectionalLight::new();
    light_component.change_color(glam::vec3(1.0, 1.0, 1.0));
    light_component.change_intensity(20.0);

    stage.manager.push(&light_2, light_component);

    let mut fps = FPS::new();
    fps._fps = 300;
    let mut global_timer = Instant::now();
    let mut start_tick = Instant::now();

    fps.fps_limit = Duration::new(0, 1000000000u32 / fps._fps);
    let mut delta_time = 1.0 / fps._fps as f32;

    if let Some(model) = stage
        .manager
        .query_entity(&game_objects[0])
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Model>()
    {
        model
            .shader
            .descriptor_manager
            .change_image_value("colorMap", &texture);
        model
            .shader
            .descriptor_manager
            .change_image_value("normalMap", &normal);
        model
            .shader
            .descriptor_manager
            .change_image_value("specularMap", &specular);
    }

    let left = Texture::new("models/cubemap/left.png".to_owned());
    let right = Texture::new("models/cubemap/right.png".to_owned());
    let down = Texture::new("models/cubemap/down.png".to_owned());
    let up: Texture = Texture::new("models/cubemap/up.png".to_owned());
    let foward = Texture::new("models/cubemap/foward.png".to_owned());
    let backward = Texture::new("models/cubemap/png.png".to_owned());

    
    if let Some(model) = stage
        .manager
        .query_entity(&skybox)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Model>()
    {
        model.shader.descriptor_manager.change_cubemap_value(
            "skybox",
            [&left, &right, &down, &up, &foward, &backward],
        );
    }


    let mut color = [0.0; 4];

    'running: loop {
        delta_time = 1.0 / ((fps.frame_count / fps.frame_elapsed) as f32);

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

        if keyboard_pool.get_key(Keycode::W) {
            camera.update_position(CameraDirection::FOWARD, delta_time);
        }
        if keyboard_pool.get_key(Keycode::S) {
            camera.update_position(CameraDirection::BACKWARD, delta_time);
        }
        if keyboard_pool.get_key(Keycode::D) {
            camera.update_position(CameraDirection::RIGHT, delta_time);
        }
        if keyboard_pool.get_key(Keycode::A) {
            camera.update_position(CameraDirection::LEFT, delta_time);
        }
        if keyboard_pool.get_key(Keycode::Space) {
            camera.update_position(CameraDirection::UP, delta_time);
        }
        if keyboard_pool.get_key(Keycode::LCtrl) {
            camera.update_position(CameraDirection::DOWN, delta_time);
        }

        /*let ctx = platform.context();
        // Draw an egui window
        egui::Window::new("Hello, world!")
            .fixed_size(egui::Vec2::new(840.0, 680.0))
            .show(&ctx, |ui| {
                ui.label("Hello, world!");
                if ui.button("Greet").clicked() {
                    println!("Hello, world!");
                }
                ui.horizontal(|ui| {
                    ui.label("Color: ");
                    ui.color_edit_button_rgba_premultiplied(&mut color);
                });
                ui.horizontal_top(|ui| ui.checkbox(&mut true, "gasfs"))
            });

        let output = platform.end_frame(&mut video).unwrap();

        for (id, texture) in output.textures_delta.set {
            println!("ID: {:?}\n{:?}", id, texture.pos);

            match texture.image {
                ImageData::Color(color_image) => {
                    println!("COLOR: {:?}\n{:?}", id, color_image.size);
                }
                ImageData::Font(font_image) => {}
            };
        }*/

        camera.update_direction(mouse_pool.get_dx(), mouse_pool.get_dy(), delta_time);

        let command_buffer = app
            .renderer
            .write()
            .unwrap()
            .begin_swapchain_command_buffer(&app.get_device(), &app.window)
            .unwrap();

        app.renderer
            .read()
            .unwrap()
            .begin_frame(&app.device, command_buffer);

        let frame_index = app.renderer.read().unwrap().get_frame_index() as usize;

        let frame_info: FrameInfo<'_> = FrameInfo {
            frame_time: 0.0,
            command_buffer,
            camera: &camera,
        };

        app.renderer
            .read()
            .unwrap()
            .begin_swapchain_renderpass(&app.device, command_buffer);


        if let Some(transform) = stage
            .manager
            .query_entity(&skybox)
            .unwrap()
            .write()
            .unwrap()
            .get_mut_component::<Transform>()
        {
            transform.translation = camera.get_position();
            println!("{:?}",transform.translation);
        }


        for (_, entity) in stage.manager.entities.write().unwrap().iter_mut() {
            let model_matrix = entity
                .write()
                .unwrap()
                .get_mut_component::<Transform>()
                .unwrap()
                .get_mat4();

            let normal_matrix = entity
                .write()
                .unwrap()
                .get_mut_component::<Transform>()
                .unwrap()
                .get_normal_matrix();

            let is = entity.read().unwrap().has_component::<Model>();

            if is {
                if let Some(cube) = entity.write().unwrap().get_mut_component::<Model>() {
                    cube.shader.descriptor_manager.change_buffer_value(
                        "GlobalUBO",
                        frame_index as u32,
                        &[camera.get_matrix()],
                    );
                    cube.shader.descriptor_manager.change_buffer_value(
                        "MaterialInfo",
                        frame_index as u32,
                        &[material],
                    );
                    cube.shader.descriptor_manager.change_buffer_value(
                        "LightInfo",
                        frame_index as u32,
                        &[raw_light_2, raw_light],
                    );

                    //renderer.render_game_objects(&device, &frame_info, &mut query, Rc::clone(&shader));

                    cube.shader
                        .pipeline
                        .as_ref()
                        .unwrap()
                        .bind(&app.device, frame_info.command_buffer);

                    unsafe {
                        app.device.device().cmd_bind_descriptor_sets(
                            frame_info.command_buffer,
                            vk::PipelineBindPoint::GRAPHICS,
                            cube.shader.pipeline_layout.unwrap(),
                            0,
                            &[cube
                                .shader
                                .descriptor_manager
                                .get_descriptor_set(frame_index as u32)],
                            &[],
                        );
                    }

                    let push = PushConstantData {
                        model_matrix,
                        normal_matrix,
                    };

                    let push_bytes: &[u8] = unsafe {
                        let struct_ptr = &push as *const _ as *const u8;
                        std::slice::from_raw_parts(
                            struct_ptr,
                            std::mem::size_of::<PushConstantData>(),
                        )
                    };

                    unsafe {
                        app.device.device().cmd_push_constants(
                            frame_info.command_buffer,
                            cube.shader.pipeline_layout.unwrap(),
                            vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                            0,
                            push_bytes,
                        );
                    }

                    cube.render(frame_info.command_buffer, &app.device);
                }
            }
        }

        app.renderer
            .read()
            .unwrap()
            .end_swapchain_renderpass(command_buffer, &app.device);

        app.renderer
            .write()
            .unwrap()
            .end_frame(&app.device, &mut app.window);

        //save_color_image_as_png(app.get_device(), (app.renderer.read().unwrap().swapchain.get_current_image(frame_index),app.renderer.read().unwrap().swapchain.get_swapchain_image_format()), (840,680),"./test.png");

        //print!("\rFPS: {:.2}", fps.frame_count / fps.frame_elapsed);
        //print!("\r{:?}",fps.frame_count / fps.frame_elapsed);
        let title = String::from("Lumina Dev App ")
            + format!("[FPS: {:.0}]", fps.frame_count / fps.frame_elapsed).as_str();
        app.window.get_window().set_title(title.as_str());
        if start_tick.elapsed() < fps.fps_limit {
            thread::sleep(fps.fps_limit - start_tick.elapsed());
        }
        fps.update();
    }
}

pub fn save_color_image_as_png(
    device: Arc<Device>,
    image: (vk::Image, vk::Format),
    size: (u32, u32),
    file_path: &str,
) {
    let buffer_size = size.0 * size.1 * 4;

    let mut buffer = Buffer::new(
        Arc::clone(&device),
        buffer_size as u64,
        1,
        vk::BufferUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    );

    DescriptorManager::transition_image_layout(
        Arc::clone(&device),
        image.0,
        image.1,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
    );

    let command_buffer: vk::CommandBuffer;

    let alloc_info = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        command_pool: device.get_command_pool(),
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: 1,
    };

    command_buffer = unsafe {
        device
            .device()
            .allocate_command_buffers(&alloc_info)
            .unwrap()[0]
    };

    let begin_info = vk::CommandBufferBeginInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next: std::ptr::null(),
        flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        ..Default::default()
    };

    unsafe {
        device
            .device()
            .begin_command_buffer(command_buffer, &begin_info)
            .expect("Failed to begin command buffer!");
    }

    let region = vk::BufferImageCopy {
        buffer_offset: 0,
        buffer_row_length: 0,
        buffer_image_height: 0,
        image_subresource: vk::ImageSubresourceLayers {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1,
        },
        image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
        image_extent: vk::Extent3D {
            width: 840,
            height: 680,
            depth: 1,
        },
    };

    unsafe {
        device.device().cmd_copy_image_to_buffer(
            command_buffer,
            image.0,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            buffer.get_buffer(),
            &[region],
        )
    }

    unsafe {
        device
            .device()
            .end_command_buffer(command_buffer)
            .expect("Failed to end command buffer!");
        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            ..Default::default()
        };
        device
            .device()
            .queue_submit(device.graphics_queue(), &[submit_info], vk::Fence::null())
            .expect("Failed to submit data");
        device
            .device()
            .queue_wait_idle(device.graphics_queue())
            .unwrap();
        device
            .device()
            .free_command_buffers(device.get_command_pool(), &[command_buffer]);
    }

    buffer.map(None, None);

    let data = buffer.convert_to_raw_data();

    //println!("{:?}",data);

    // Create an empty ImageBuffer
    let mut image_buffer = ImageBuffer::new(840, 680);

    // Copy the pixels from the ColorImage into the ImageBuffer
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let index = (y * 1024 + x) as usize;
        let depth_value = data[index];
        *pixel = Luma([depth_value]);
    }

    // Convert the ImageBuffer into a DynamicImage
    let dynamic_image: DynamicImage = DynamicImage::ImageLuma8(image_buffer);

    // Save the DynamicImage as a PNG file
    dynamic_image.save(file_path).unwrap();
}

/*fn main() {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        std::env::set_var("SDL_VIDEODRIVER", "wayland");
    }

    let sdl_context = sdl2::init().unwrap();

    let mut window = Window::new(&sdl_context, "Lumina Dev App", 800, 640);
    let device = Rc::new(Device::new(&window));

    let mut query = Query::new();

    let window_icon = sdl2::surface::Surface::from_file("icons/LuminaLogoMain.png").unwrap();

    window._window.set_icon(window_icon);

    sdl_context.mouse().set_relative_mouse_mode(true);

    let mut shader = Shader::new(
        Rc::clone(&device),
        "shaders/default_shader.vert",
        "shaders/default_shader.frag",
    );

    let mut renderer: Renderer = Renderer::new(&window, &device, None);
    //renderer.activate_shader(&device, &shader);

    let mut keyboard_pool = Keyboard::new();

    let mut mouse_pool = Mouse::new();

    let mut game_objects: Vec<GameObject> = Vec::new();

    let cube_positions: [glam::Vec3; 10] = [
        glam::Vec3::new(0.0, 0.0, 0.0),
        glam::Vec3::new(2.0, 5.0, -15.0),
        glam::Vec3::new(-1.5, -2.2, -2.5),
        glam::Vec3::new(-3.8, -2.0, -12.3),
        glam::Vec3::new(2.4, -0.4, -3.5),
        glam::Vec3::new(-1.7, 3.0, -7.5),
        glam::Vec3::new(1.3, -2.0, -2.5),
        glam::Vec3::new(1.5, 2.0, -2.5),
        glam::Vec3::new(1.5, 0.2, -1.5),
        glam::Vec3::new(-1.3, 1.0, -1.5),
    ];

    for i in 0..10 {
        let cube = shapes::cube(&mut query, Rc::clone(&device));

        if let Some(transform) = query.query_mut::<Transform>(&cube) {
            transform.translation = cube_positions[i];
            transform.scale = glam::vec3(1.0, 1.0, 1.0);
            let angle = 20.0 * i as f32;
            transform.rotation = glam::vec3(angle, angle, angle);
        }

        game_objects.push(cube);
    }

    let mut cube = shapes::model_cube(&mut query, Rc::clone(&device));

    let mut camera = Camera::new(renderer.get_aspect_ratio(), false);

    let mut view = Transform::default();
    view.translation = glam::Vec3::ONE;
    view.rotation = glam::Vec3::ZERO;

    let aspect = renderer.get_aspect_ratio();

    let mut time: f32 = 0.0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    if let Some(cube) = query.query_mut::<Model>(&game_objects[0]) {
        cube.shader.create_pipeline_layout(true);
        cube.shader
            .create_pipeline(renderer.get_swapchain_renderpass());
    }

    'running: loop {

        if keyboard_pool.get_key(Keycode::Escape) {
            break 'running;
        }

        if let Some(command_buffer) = renderer.begin_frame(&device, &window) {
            let frame_index = renderer.get_frame_index() as usize;

            let frame_info: FrameInfo<'_> = FrameInfo {
                frame_time: 0.0,
                command_buffer,
                camera: &camera,
                global_descriptor_set: shader
                    .descriptor_manager
                    .get_descriptor_set(frame_index as u32),
            };

            renderer.begin_swapchain_renderpass(command_buffer, &device);

            let ubo: GlobalUBO = GlobalUBO {
                projection: camera.get_matrix(),
            };

            let material: MaterialInfo = MaterialInfo {
                material: Material {
                    ambient: [0.1, 0.1, 0.1],
                    diffuse: [0.0, 0.0, 0.0],
                    specular: [0.5, 0.5, 0.5],
                    shininess: 1.0,
                    _padding1: [0.0],
                    _padding2: [0.0],
                },
                view_pos: camera.get_position().to_array(),
            };

            let light: LightInfo = LightInfo {
                light: SpotLight {
                    position: [1.2, 1.0, 2.0],
                    direction: [-0.6, 0.0, -0.9],
                    cut_off: 12.5.to_radians(),
                    outer_cut_off: 17.5.to_radians(),
                    ambient: [0.1, 0.1, 0.1],
                    diffuse: [0.0, 0.0, 0.0],
                    specular: [0.25, 0.25, 0.25],
                    constant: 1.0,
                    linear: 0.7,
                    quadratic: 1.8,
                    _padding1: 0.0,
                    _padding2: 0.0,
                    _padding3: 0.0,
                    _padding4: 0.0,
                    _padding5: 0.0,
                    _padding6: 0.0,
                    _padding7: 0.0,
                    _padding8: 0.0,
                    _padding9: 0.0,
                },
            };

            let texture = Texture::new(String::default(), Rc::clone(&device));

            let model_matrix = query
                .query::<Transform>(&game_objects[0])
                .unwrap()
                .get_mat4();
            let normal_matrix = query
                .query::<Transform>(&game_objects[0])
                .unwrap()
                .get_normal_matrix();

            if let Some(cube) = query.query_mut::<Model>(&game_objects[0]) {
                cube.shader.descriptor_manager.change_buffer_value(
                    "GlobalUBO".to_string(),
                    frame_index as u32,
                    &[ubo],
                );
                cube.shader.descriptor_manager.change_buffer_value(
                    "MaterialInfo".to_string(),
                    frame_index as u32,
                    &[material],
                );
                cube.shader.descriptor_manager.change_buffer_value(
                    "LightInfo".to_string(),
                    frame_index as u32,
                    &[light],
                );
                cube.shader.descriptor_manager.change_image_value(
                    "normalMap".to_string(),
                    frame_index as u32,
                    texture,
                );

                //renderer.render_game_objects(&device, &frame_info, &mut query, Rc::clone(&shader));

                cube.shader
                    .pipeline
                    .as_ref()
                    .unwrap()
                    .bind(&device, frame_info.command_buffer);

                unsafe {
                    device.device().cmd_bind_descriptor_sets(
                        frame_info.command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        cube.shader.pipeline_layout.unwrap(),
                        0,
                        &[cube.shader.descriptor_manager.get_descriptor_set(frame_index as u32)],
                        &[],
                    );
                }

                let push = PushConstantData {
                    model_matrix,
                    normal_matrix,
                };

                let push_bytes: &[u8] = unsafe {
                    let struct_ptr = &push as *const _ as *const u8;
                    std::slice::from_raw_parts(struct_ptr, std::mem::size_of::<PushConstantData>())
                };

                unsafe {
                    device.device().cmd_push_constants(
                        frame_info.command_buffer,
                        cube.shader.pipeline_layout.unwrap(),
                        vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                        0,
                        push_bytes,
                    );
                }

                cube.mesh.bind(command_buffer, &device);
                cube.mesh.draw(command_buffer, &device);
            }
            renderer.end_swapchain_renderpass(command_buffer, &device);
        }

        renderer.end_frame(&device, &mut window);

    }
}*/
