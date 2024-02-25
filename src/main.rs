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
use egui::{epaint::Primitive, ColorImage, ImageData, Key};
use glsl_parser::parser::Parser;
use image::{DynamicImage, ImageBuffer, Luma, Rgba};
use lumina_atlas::atlas::Atlas;
use lumina_ecs::{app::App, query::Query, stage::Stage};
use rand::Rng;

use lumina_core::{
    device::Device, fps_manager::FPS, swapchain::Swapchain, texture::Texture, window::Window,
    Vertex2D,
};

use lumina_data::{
    buffer::Buffer,
    descriptor::{DescriptorPool, DescriptorSetLayout, DescriptorWriter, LayoutConfig, PoolConfig},
    descriptor_manager::{self, DescriptorManager},
};
use lumina_geometry::shapes;
use lumina_graphic::shader::{PushConstantData, Shader};
use lumina_input::{
    keyboard::{Keyboard, Keycode},
    mouse::{Mouse, MouseButton},
};
use lumina_object::{
    game_object::{self, Component, GameObject},
    transform::Transform,
};
use lumina_render::{
    camera::{Camera, CameraDirection},
    gui_canvas::GuiCanvas,
    model::Model,
    renderer::Renderer,
};
//use lumina_pbr::material::Material;

use sdl2::{event::Event, Sdl};

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

    let mut stage = Stage::new("hfd");

    let instant = Instant::now();

    let mut tex_sky_texture = Texture::new_raw("models/cubemap/top.jpg");
    let mut tex_texture = Texture::new_raw("models/Character_baseColor.jpeg");
    let mut tex_paving = Texture::new_raw("models/PavingStone_Color.png");
        
    //app.load_file("hfd.lumin");

    
    /*let mut atlas = Atlas::new();

    atlas.pack_textures(vec![&mut tex_sky_texture,&mut tex_texture,&mut tex_paving,&mut tex_sky_texture_2,&mut tex_texture_2,&mut tex_paving_2]);
    
    println!("{:?}",atlas.images);

    atlas.texture.save("test.png").unwrap();

    panic!("took: {:?} seconds",instant.elapsed().as_secs_f64());*/
    
    let model = Model::new_from_model(app.get_device(), "models/men.gltf");

    //println!("{:?}",model.convert_to_json());

    let low_poly: GameObject = stage.manager.spawn();

    stage.manager.push(&low_poly, model);

    //app.save_scene();
    stage.save_scene();

    app.load_file("./hfd.lumin");

    if let Some(transform) = stage
        .manager
        .query_entity(&low_poly)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Transform>()
    {
        transform.translation = glam::Vec3::new(0.0, 0.0, 5.0);
        transform.scale = glam::vec3(1.0, 1.0, 1.0);
        let angle = 20.0 * 0 as f32;
        transform.rotation = glam::vec3(-90.0, angle, angle);
    }

    if let Some(model) = stage
        .manager
        .query_entity(&low_poly)
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
        /*model
            .shader
            .descriptor_manager
            .change_image_size("normalMap", 1024, 1024);
        model
            .shader
            .descriptor_manager
            .change_image_size("specularMap", 1024, 1024);*/
        model
            .shader
            .descriptor_manager
            .change_buffer_count("LightInfo", 3);

        model.shader.descriptor_manager.update_we();

        //model.shader.renovate_pipeline(app.renderer.read().unwrap().get_swapchain_renderpass());
    }

    let model = shapes::model_cube(app.get_device());

    let cube: GameObject = stage.manager.spawn();

    stage.manager.push(&cube, model);

    if let Some(transform) = stage
        .manager
        .query_entity(&cube)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Transform>()
    {
        transform.translation = glam::Vec3::new(0.0, 0.0, 5.0);
        transform.scale = glam::vec3(100.0, 0.0, 100.0);
        let angle = 20.0 * 0 as f32;
        transform.rotation = glam::vec3(0.0, angle, angle);
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
            .change_image_size("colorMap", 2048, 2048);
        /* model
            .shader
            .descriptor_manager
            .change_image_size("normalMap", 2048, 2048);
        model
            .shader
            .descriptor_manager
            .change_image_size("specularMap", 2048, 2048);*/
        model
            .shader
            .descriptor_manager
            .change_buffer_count("LightInfo", 3);

        model.shader.descriptor_manager.update_we();

        //model.shader.renovate_pipeline(app.renderer.read().unwrap().get_swapchain_renderpass());
    }

    let mut model = shapes::model_cube(app.get_device());

    let skybox = stage.manager.spawn();

    model.shader = Shader::new(
        app.get_device(),
        "shaders/skybox/skybox_shader.vert",
        "shaders/skybox/skybox_shader.frag",
        lumina_core::Vertex3D::setup(),
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
            .change_image_size("skybox", 2048, 2048);
        model.shader.descriptor_manager.update_we();

        //model.shader.renovate_pipeline(app.renderer.read().unwrap().get_swapchain_renderpass());
    }

    let mut camera = Camera::new(app.renderer.read().unwrap().get_aspect_ratio(), false);
    let gui_camera = Camera::new(app.renderer.read().unwrap().get_aspect_ratio(), true);

    camera.speed = 10.0;

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

    let raw_light_3: LightInfo = LightInfo {
        light: RawLight {
            position: [0.0, -5.0, 5.0],
            rotation: [-0.6, 90.0, -5.9],
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            spot_size: 7.5,
            linear: 0.7,
            quadratic: 1.8,
            light_type: 0,
            _padding1: 0,
            _padding2: 0,
        },
    };

    let raw_light_2: LightInfo = LightInfo {
        light: RawLight {
            position: [5.0, -1.0, 0.0],
            rotation: [-0.0, 0.0, -0.0],
            color: [1.0, 1.0, 0.0],
            intensity: 20.0,
            spot_size: 12.0,
            linear: 0.7,
            quadratic: 1.8,
            light_type: 1,
            _padding1: 0,
            _padding2: 0,
        },
    };

    let raw_light: LightInfo = LightInfo {
        light: RawLight {
            position: [0.0, -5.0, 5.0],
            rotation: [-0.6, 90.0, -5.9],
            color: [1.0, 1.0, 1.0],
            intensity: 1000.0,
            spot_size: 7.5,
            linear: 0.7,
            quadratic: 1.8,
            light_type: 2,
            _padding1: 0,
            _padding2: 0,
        },
    };

    let sky_texture = tex_sky_texture.create_texture();
    let texture = tex_texture.create_texture();
    let paving = tex_paving.create_texture();

    let mut gui_canvas = GuiCanvas::new(app.get_device(),app.renderer.read().unwrap().get_swapchain_renderpass());

    let mut fps = FPS::new();
    fps._fps = 300;
    let mut global_timer = Instant::now();
    let mut start_tick = Instant::now();

    fps.fps_limit = Duration::new(0, 1000000000u32 / fps._fps);
    let mut delta_time = 1.0 / fps._fps as f32;

    if let Some(model) = stage
        .manager
        .query_entity(&low_poly)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Model>()
    {
        model
        .shader
        .descriptor_manager
        .change_image_value("colorMap", &texture);
        /*model
            .shader
            .descriptor_manager
            .change_image_value("normalMap", &normal);
        model
            .shader
            .descriptor_manager
            .change_image_value("specularMap", &specular);*/
    }

    if let Some(model) = stage
        .manager
        .query_entity(&cube)
        .unwrap()
        .write()
        .unwrap()
        .get_mut_component::<Model>()
    {
        model
        .shader
        .descriptor_manager
        .change_image_value("colorMap", &paving);
        /*model
            .shader
            .descriptor_manager
            .change_image_value("normalMap", &normal);
        model
            .shader
            .descriptor_manager
            .change_image_value("specularMap", &specular);*/
    }

    /*let left = Texture::new("models/cubemap/left.jpg".to_owned());
    let right = Texture::new("models/cubemap/right.jpg".to_owned());
    let down = Texture::new("models/cubemap/bottom.jpg".to_owned());
    let up: Texture = Texture::new("models/cubemap/top.jpg".to_owned());
    let foward = Texture::new("models/cubemap/front.jpg".to_owned());
    let backward = Texture::new("models/cubemap/back.jpg".to_owned());*/

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
            [
                &sky_texture,
                &sky_texture,
                &sky_texture,
                &sky_texture,
                &sky_texture,
                &sky_texture,
            ],
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
            camera.update_position(CameraDirection::FORWARD, delta_time);
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

        let ctx = platform.context();
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
        let paint_jobs = platform.tessellate(&output);
        let pj = paint_jobs.as_slice();

        for (id, texture) in output.textures_delta.set {
            println!("ID: {:?}\n{:?}", id, texture.pos);

            match texture.image {
                ImageData::Color(color_image) => {
                    println!("COLOR: {:?}\n{:?}", id, color_image.pixels);
                }
                ImageData::Font(font_image) => {
                    println!("FONT: {:?}\n{:?}", id, font_image.size);
                }
            };
        }

        camera.update_direction(mouse_pool.get_dx(), mouse_pool.get_dy(), delta_time);

        if let Some(transform) = stage
            .manager
            .query_entity(&skybox)
            .unwrap()
            .write()
            .unwrap()
            .get_mut_component::<Transform>()
        {
            transform.translation = camera.get_position();
        }

        let command_buffer = app
            .renderer
            .write()
            .unwrap()
            .begin_swapchain_command_buffer(&app.device, &app.window)
            .unwrap();

        app.renderer
            .read()
            .unwrap()
            .begin_frame(&app.device, command_buffer);

        app.renderer
            .read()
            .unwrap()
            .begin_swapchain_renderpass(&app.device, command_buffer);

        stage.render(
            Arc::clone(&app.renderer),
            app.get_device(),
            command_buffer,
            camera,
        );

        /*let mut vertices: Vec<Vec<Vertex2D>> = Vec::new();
        let mut indices: Vec<Vec<u32>> = Vec::new();

        for clipped in pj {
            match &clipped.primitive {
                Primitive::Mesh(mesh) => {
                    let mut temp_verts = Vec::new();

                    for vertice in mesh.vertices.iter() {
                        temp_verts.push(Vertex2D {
                            position: glam::vec2(vertice.pos.x, vertice.pos.y),
                            color: glam::vec4(
                                vertice.color.r() as f32,
                                vertice.color.g() as f32,
                                vertice.color.b() as f32,
                                vertice.color.a() as f32,
                            ),
                            uv: glam::vec2(vertice.uv.x, vertice.uv.y),
                        });
                    }

                    vertices.push(temp_verts);
                    indices.push(mesh.indices.clone());
                }
                _ => {
                    println!("It's the other shit lol")
                }
            };
            //panic!("");
        }

        gui_canvas.render(
            command_buffer,
            vertices,
            indices,
            app.renderer.read().unwrap().get_frame_index() as u32,
            gui_camera,
        );*/

        if keyboard_pool.get_key(Keycode::F12) {
            save_color_image_as_png(
                app.get_device(),
                (
                    app.renderer
                        .read()
                        .unwrap()
                        .swapchain
                        .get_current_image(app.renderer.read().unwrap().get_frame_index() as usize),
                    app.renderer
                        .read()
                        .unwrap()
                        .swapchain
                        .get_swapchain_image_format(),
                ),
                (1280, 720),
                "./test.png",
            );
        }

        app.renderer
            .read()
            .unwrap()
            .end_swapchain_renderpass(command_buffer, &app.device);

        app.renderer
            .write()
            .unwrap()
            .end_frame(&app.device, &mut app.window);

        let title = String::from("Lumina Dev App ")
            + format!("[FPS: {:.0}]", fps.frame_count / fps.frame_elapsed).as_str();
        app.window.get_window().set_title(title.as_str()).unwrap();
        if start_tick.elapsed() < fps.fps_limit {
            let sleep_duration =
                if let Some(remaining) = fps.fps_limit.checked_sub(start_tick.elapsed()) {
                    remaining
                } else {
                    Duration::from_secs(0)
                };

            thread::sleep(sleep_duration);
        }
        fps.update();
    }

    app.drop();
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
            width: size.0,
            height: size.1,
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

    let mut image_buffer = ImageBuffer::<Rgba<u8>, _>::new(size.0, size.1);

    // Copy the pixels from the buffer into the ImageBuffer
    for y in 0..size.1 {
        for x in 0..size.0 {
            let index = (y * size.0 + x) as usize * 4;
            let r = data[index];
            let g = data[index + 1];
            let b = data[index + 2];
            let a = data[index + 3];
            *image_buffer.get_pixel_mut(x, y) = Rgba([b, g, r, a]); // Invert the order of R, G, B
        }
    }

    // Convert the ImageBuffer into a DynamicImage
    let dynamic_image: DynamicImage = DynamicImage::ImageRgba8(image_buffer);

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
