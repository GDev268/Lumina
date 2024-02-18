use std::{
    any::TypeId,
    borrow::BorrowMut,
    collections::HashMap,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Barrier, Mutex, RwLock},
    thread::{self, JoinHandle},
};

use ash::vk;
use lumina_bundle::{RendererBundle, ResourcesBundle};
use lumina_core::{
    device::Device, framebuffer::Framebuffer, image::Image, window::Window, RawLight,
};
use lumina_graphic::shader::{PushConstantData, Shader};
use lumina_object::{game_object::GameObject, transform::Transform};
use lumina_pbr::light::{DirectionalLight, Light};
use lumina_render::{camera::Camera, model::Model, renderer::Renderer};

use crate::query::Query;

/*use lumina_object::{
    component_manager::{self, ComponentManager},
    entity::Entity,
    game_object::{Component, GameObject},
    transform::Transform,
};
use lumina_pbr::light::{DirectionalLight, PointLight, SpotLight};
use lumina_render::{camera::Camera, model::Model};
use rand::Rng;*/

pub struct Stage {
    name: String,
    pub manager: Query,
}

impl Stage {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            manager: Query::new(),
        }
    }

    pub fn create_directional_shadow_maps(
        &mut self,
        lights: Arc<Vec<GameObject>>,
        render_pass: vk::RenderPass,
        renderer: Arc<RwLock<Renderer>>,
        device: Arc<Device>,
    ) -> Vec<Image> {
        let dir_lights = Arc::new(lights);

        let num_cpus = num_cpus::get();

        let mut shadow_maps: Vec<Image> = Vec::new();

        let barrier = Arc::new(Barrier::new(num_cpus + 1));

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let renderer_clone = renderer.clone();
                let device_clone = device.clone();
                let manager_clone = self.manager.entities.clone();
                let lights_clone = dir_lights.clone();
                let barrier_clone = barrier.clone();

                thread::spawn(move || {
                    let size = lights_clone.len();
                    let start = i * size / num_cpus;
                    let end = if i == num_cpus - 1 {
                        size
                    } else {
                        (i + 1) * size / num_cpus
                    };

                    let mut shadow_maps: Vec<Image> = Vec::new();

                    let mut color_images = Vec::new();
                    let mut depth_images = Vec::new();
                    let mut framebuffers = Vec::new();

                    for light in lights_clone.iter().skip(start).take(end - start) {
                        let mut position = glam::Vec3::ZERO;
                        if let Some(transform) = manager_clone
                            .write()
                            .unwrap()
                            .get_mut(&light.get_id())
                            .unwrap()
                            .write()
                            .unwrap()
                            .get_mut_component::<Transform>()
                        {
                            position = transform.translation;
                        }

                        let projection = Camera::create_orthographic_projection(
                            -10.0, 10.0, -10.0, 10.0, 1.0, 1000.0,
                        );

                        let look_projection = glam::Mat4::look_at_lh(
                            position,
                            glam::Vec3::ZERO,
                            glam::vec3(0.0, 1.0, 0.0),
                        );

                        let final_projection = projection * look_projection;

                        let mut shader = Shader::new(
                            device_clone.clone(),
                            "shaders/shadow_map_shader.vert",
                            "shaders/shadow_map_shader.frag",
                        );
                        shader.create_pipeline_layout(true);
                        shader.create_pipeline(render_pass);

                        let alloc_info = vk::CommandBufferAllocateInfo {
                            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                            p_next: std::ptr::null(),
                            level: vk::CommandBufferLevel::PRIMARY,
                            command_pool: device_clone.get_command_pool(),
                            command_buffer_count: 1,
                        };

                        let command_buffer = unsafe {
                            device_clone
                                .device()
                                .allocate_command_buffers(&alloc_info)
                                .expect("Failed to allocate command buffers!")[0]
                        };

                        let mut color_image = Image::new_2d(
                            &device_clone,
                            vk::Format::B8G8R8A8_SRGB,
                            vk::ImageUsageFlags::COLOR_ATTACHMENT,
                            vk::MemoryPropertyFlags::DEVICE_LOCAL,
                            1024,
                            1024,
                        );

                        color_image.new_image_view(&device_clone, vk::ImageAspectFlags::COLOR);

                        let mut depth_image = Image::new_2d(
                            &device_clone,
                            vk::Format::D32_SFLOAT,
                            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                            vk::MemoryPropertyFlags::DEVICE_LOCAL,
                            1024,
                            1024,
                        );

                        depth_image.new_image_view(&device_clone, vk::ImageAspectFlags::DEPTH);

                        let framebuffer = Framebuffer::new(
                            &device_clone,
                            [color_image.get_image_view(), depth_image.get_image_view()],
                            render_pass,
                            1024,
                            1024,
                        );

                        renderer_clone
                            .read()
                            .unwrap()
                            .begin_frame(&device_clone, command_buffer);
                        renderer_clone.read().unwrap().begin_custom_renderpass(
                            &device_clone,
                            command_buffer,
                            vk::Extent2D {
                                width: 1024,
                                height: 1024,
                            },
                            &framebuffer,
                        );

                        shader.descriptor_manager.change_buffer_value(
                            "GlobalUBO",
                            0,
                            &[final_projection.to_cols_array_2d()],
                        );

                        shader
                            .pipeline
                            .as_ref()
                            .unwrap()
                            .bind(&device_clone, command_buffer);

                        for (id, entity) in manager_clone.read().unwrap().iter() {
                            let entity_lock = entity.read().unwrap();
                            if entity_lock.has_component::<Model>() {
                                let model_matrix =
                                    entity_lock.get_component::<Transform>().unwrap().get_mat4();

                                let normal_matrix = entity_lock
                                    .get_component::<Transform>()
                                    .unwrap()
                                    .get_normal_matrix();

                                if let Some(model) = entity_lock.get_component::<Model>() {
                                    unsafe {
                                        device_clone.device().cmd_bind_descriptor_sets(
                                            command_buffer,
                                            vk::PipelineBindPoint::GRAPHICS,
                                            shader.pipeline_layout.unwrap(),
                                            0,
                                            &[shader.descriptor_manager.get_descriptor_set(0)],
                                            &[],
                                        );

                                        let push = PushConstantData {
                                            model_matrix,
                                            normal_matrix,
                                        };

                                        let push_bytes: &[u8] = {
                                            let struct_ptr = &push as *const _ as *const u8;
                                            std::slice::from_raw_parts(
                                                struct_ptr,
                                                std::mem::size_of::<PushConstantData>(),
                                            )
                                        };

                                        device_clone.device().cmd_push_constants(
                                            command_buffer,
                                            shader.pipeline_layout.unwrap(),
                                            vk::ShaderStageFlags::VERTEX
                                                | vk::ShaderStageFlags::FRAGMENT,
                                            0,
                                            push_bytes,
                                        );
                                    }
                                }
                            }
                        }

                        unsafe {
                            device_clone.device().cmd_end_render_pass(command_buffer);
                            device_clone
                                .device()
                                .end_command_buffer(command_buffer)
                                .unwrap();

                            let submit_info: vk::SubmitInfo = vk::SubmitInfo {
                                s_type: vk::StructureType::SUBMIT_INFO,
                                p_next: std::ptr::null(),
                                command_buffer_count: 1,
                                p_command_buffers: &command_buffer,
                                ..Default::default()
                            };

                            device_clone
                                .device()
                                .queue_submit(
                                    device_clone.graphics_queue(),
                                    &[submit_info],
                                    vk::Fence::null(),
                                )
                                .expect("Failed to submit draw command buffer!");

                            device_clone.device().free_command_buffers(
                                device_clone.get_command_pool(),
                                &[command_buffer],
                            );

                            shader.destroy(&device_clone);
                            drop(shader);
                        };

                        color_images.push(color_image);
                        depth_images.push(depth_image);
                        framebuffers.push(framebuffer);

                        //println!("{:?}", depth_image);
                        //shadow_maps.push(depth_image);
                    }


                    for mut color in color_images {
                        color.clean_memory(&device_clone);
                        color.clean_image(&device_clone);
                        color.clean_view(&device_clone);
                        drop(color);
                    }



                    for depth in depth_images {
                        shadow_maps.push(depth);
                    }

                    for mut framebuffer in framebuffers {
                        framebuffer.clean_framebuffer(&device_clone);
                        drop(framebuffer);
                    }


                    barrier_clone.wait();
                    return shadow_maps;
                })
            })
            .collect();

        barrier.wait();

        for handle in handles {
            shadow_maps.extend(handle.join().unwrap());
        }

        return shadow_maps;
    }

    /*pub fn create(
        &mut self,
        device: Rc<Device>,
        aspect_ratio: f32,
        window: &Window,
        renderer_bundle: &RendererBundle,
    ) {
        let camera = self.manager.spawn();

        let camera_component = Camera::new(
            aspect_ratio,
            false,
        );

        self.manager.push(&camera, camera_component);

        self.cameras.write().unwrap().push(camera);
    }*/

    /*pub fn update(
        &mut self,
        resources: Arc<RwLock<ResourcesBundle>>,
        fps: f32,
    ) {
        let delta_time = 1.0 / fps;
        let num_cpus = num_cpus::get().max(1);

        self.manager.query_all_components();
        let components_clone_snapshot = Arc::clone(&self.manager.components_snapshot);
        let components_clone = Arc::clone(&self.manager.components);

        let resources_lock = Arc::clone(&resources);

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let locked_resources = Arc::clone(&resources_lock);
                let locked_components_snapshot = Arc::clone(&components_clone_snapshot);
                let thread_components = Arc::clone(&components_clone);

                thread::spawn(move || {
                    for (id, component_group) in thread_components.write().unwrap().iter_mut() {
                        for (type_id, component) in component_group.iter_mut() {
                            component.update(*id,&locked_components_snapshot, &locked_resources);
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        drop(components_clone_snapshot);
    }

    fn test(manager: &Arc<RwLock<HashMap<u32, HashMap<TypeId, Box<(dyn lumina_object::game_object::Component + 'static)>>>>>) {
        println!("{:?}",manager.read().unwrap().get(&0).is_some());
    }

    pub fn draw(
        &mut self,
        resources: Arc<RwLock<ResourcesBundle>>,
        cur_frame: u32,
        wait_semaphore: vk::Semaphore,
        command_buffer:vk::CommandBuffer
    ) {
        let num_cpus = num_cpus::get().max(1);

        let handles: Vec<JoinHandle<()>> = (0..num_cpus)
            .map(|i| {
                let cameras_clone = Arc::clone(&self.cameras);
                let components_clone = Arc::clone(&self.manager.components);
                let resources_clone = Arc::clone(&resources);

                thread::spawn(move || {
                    let mut cameras_lock = cameras_clone.write().unwrap();
                    let mut resources_lock = resources_clone.write().unwrap();

                    let size = cameras_lock.len();
                    let start = i * size / num_cpus;
                    let end = if i == num_cpus - 1 {
                        size
                    } else {
                        (i + 1) * size / num_cpus
                    };

                    for camera in cameras_lock.iter_mut().skip(start).take(end - start) {

                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn draw_components(
        manager: Arc<RwLock<HashMap<u32, HashMap<TypeId, Box<dyn Component>>>>>,
        resources: Arc<RwLock<ResourcesBundle>>,
    ) {
        let num_cpus = num_cpus::get().max(1);

        let handles: Vec<JoinHandle<()>> = (0..num_cpus)
            .map(|i| {
                let components_clone = Arc::clone(&manager);
                let resources_clone = Arc::clone(&resources);

                thread::spawn(move || {

                    let size = components_clone.read().unwrap().len();
                    let start = i * size / num_cpus;
                    let end = if i == num_cpus - 1 {
                        size
                    } else {
                        (i + 1) * size / num_cpus
                    };

                    for (id, components) in components_clone.write().unwrap()
                        .iter_mut()
                        .skip(start)
                        .take(end - start)
                    {
                        for (type_id, component) in components.iter_mut() {
                            component.render(
                                *id,
                                Arc::clone(&components_clone),
                                Arc::clone(&resources_clone),
                            );
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

    }

    pub fn get_raw_lights(&self) -> Vec<RawLight> {
        let mut raw_lights: Vec<RawLight> = Vec::new();

        let num_cpus = num_cpus::get().max(1);

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let manager_read_lock =
                    Arc::clone(&self.manager.components);
                thread::spawn(move || {
                    let len = manager_read_lock.read().unwrap().len();
                    let start = i * len / num_cpus;
                    let end = if i == num_cpus - 1 {
                        len
                    } else {
                        (i + 1) * len / num_cpus
                    };

                    let mut local_raw_lights: Vec<RawLight> = Vec::new();

                    for (id, component_group) in manager_read_lock
                        .write()
                        .unwrap()
                        .iter_mut()
                        .skip(start)
                        .take(end - start)
                    {
                        for (type_id, component) in component_group.iter_mut() {
                            match *type_id {
                                t if t == TypeId::of::<DirectionalLight>() => local_raw_lights
                                    .push(
                                        component
                                            .as_mut_any()
                                            .downcast_mut::<DirectionalLight>()
                                            .unwrap()
                                            .create_raw_light(
                                                id,
                                                manager_read_lock
                                                    .read()
                                                    .unwrap()
                                                    .get(&id)
                                                    .unwrap()
                                                    .get(&TypeId::of::<Transform>())
                                                    .unwrap()
                                                    .as_any()
                                                    .downcast_ref::<Transform>()
                                                    .unwrap(),
                                            ),
                                    ),

                                t if t == TypeId::of::<PointLight>() => local_raw_lights.push(
                                    component
                                        .as_mut_any()
                                        .downcast_mut::<PointLight>()
                                        .unwrap()
                                        .create_raw_light(
                                            id,
                                            manager_read_lock
                                                .read()
                                                .unwrap()
                                                .get(&id)
                                                .unwrap()
                                                .get(&TypeId::of::<Transform>())
                                                .unwrap()
                                                .as_any()
                                                .downcast_ref::<Transform>()
                                                .unwrap(),
                                        ),
                                ),

                                t if t == TypeId::of::<SpotLight>() => local_raw_lights.push(
                                    component
                                        .as_mut_any()
                                        .downcast_mut::<SpotLight>()
                                        .unwrap()
                                        .create_raw_light(
                                            id,
                                            manager_read_lock
                                                .read()
                                                .unwrap()
                                                .get(&id)
                                                .unwrap()
                                                .get(&TypeId::of::<Transform>())
                                                .unwrap()
                                                .as_any()
                                                .downcast_ref::<Transform>()
                                                .unwrap(),
                                        ),
                                ),
                                _ => {}
                            }
                        }
                    }

                    local_raw_lights
                })
            })
            .collect();

        for handle in handles {
            raw_lights.extend(handle.join().unwrap());
        }

        raw_lights
    }

    pub async fn adfsasd(&self) {}*/
}

unsafe impl Send for Stage {}
