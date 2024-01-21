use std::{
    any::TypeId,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
    thread::{self, JoinHandle}, borrow::BorrowMut,
};

use ash::vk;
use lumina_bundle::{RendererBundle, ResourcesBundle};
use lumina_core::{device::Device, window::Window, RawLight};
use lumina_object::{
    component_manager::{self, ComponentManager},
    entity::Entity,
    game_object::{Component, GameObject},
    transform::Transform,
};
use lumina_pbr::light::{DirectionalLight, PointLight, SpotLight};
use lumina_render::camera::Camera;
use rand::Rng;

pub struct Stage {
    name: String,
    manager: Arc<Mutex<ComponentManager>>,
    cameras: Arc<RwLock<Vec<GameObject>>>,
}

impl Stage {
    pub fn new(name: String) -> Self {
        Self {
            name,
            manager: Arc::new(Mutex::new(ComponentManager::new())),
            cameras: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn create(
        &mut self,
        device: Rc<Device>,
        aspect_ratio: f32,
        window: &Window,
        renderer_bundle: &RendererBundle,
    ) {
        let camera = self.manager.lock().unwrap().borrow_mut().spawn();

        let camera_component = Camera::new(
            device,
            aspect_ratio,
            false,
            window.get_extent(),
            renderer_bundle,
        );

        self.manager.lock().unwrap().borrow_mut().push(&camera, camera_component);

        self.cameras.write().unwrap().push(camera);

    }

    pub fn update(
        &mut self,
        resources: Arc<RwLock<ResourcesBundle>>,
        fps: f32,
    ) {
        let delta_time = 1.0 / fps;
        
        let num_cpus = num_cpus::get().max(1);

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let manager_read_lock = Arc::clone(&self.manager.lock().unwrap().borrow_mut().components);
                let locked_resources = Arc::clone(&resources);

                locked_resources.write().unwrap().raw_lights = self.get_raw_lights();

                thread::spawn(move || {
                    let len = manager_read_lock.read().unwrap().len();
                    let start = i * len / num_cpus;
                    let end = if i == num_cpus - 1 {
                        len
                    } else {
                        (i + 1) * len / num_cpus
                    };
                    
                    for (id, component_group) in manager_read_lock
                        .write()
                        .unwrap()
                        .iter_mut()
                        .skip(start)
                        .take(end - start)
                    {
                        for (type_id, component) in component_group.iter_mut() {
                            component.update(*id, Arc::clone(&manager_read_lock), &locked_resources);
                            println!("asdsad");
                        }
                    }
                })
            })
            .collect();
    }

    pub fn draw(&mut self, resources: Arc<RwLock<ResourcesBundle>>, cur_frame:u32, wait_semaphore:vk::Semaphore) {
        let num_cpus = num_cpus::get().max(1);

        let handles: Vec<JoinHandle<()>> = (0..num_cpus)
            .map(|i| {
                let cameras_clone = Arc::clone(&self.cameras);
                let components = Arc::clone(&self.manager.lock().unwrap().components);
                let resources_clone = Arc::clone(&resources);

                thread::spawn(move || {
                    let mut cameras_lock = cameras_clone.write().unwrap();
                    let mut components_lock = components.write().unwrap();

                    let size = cameras_lock.len();
                    let start = i * size / num_cpus;
                    let end = if i == num_cpus - 1 {
                        size
                    } else {
                        (i + 1) * size / num_cpus
                    };

                    for camera in cameras_lock.iter_mut().skip(start).take(end - start) {
                        resources_clone.write().unwrap().cur_projection = components
                            .read()
                            .unwrap()
                            .get(&camera.get_id())
                            .unwrap()
                            .get(&TypeId::of::<Camera>())
                            .unwrap()
                            .as_any()
                            .downcast_ref::<Camera>()
                            .unwrap()
                            .get_matrix();
                        
                        resources_clone.write().unwrap().command_buffer = components
                        .read()
                        .unwrap()
                        .get(&camera.get_id())
                        .unwrap()
                        .get(&TypeId::of::<Camera>())
                        .unwrap()
                        .as_any()
                        .downcast_ref::<Camera>()
                        .unwrap()
                        .get_command_buffer();

                        let camera_component = components_lock.get_mut(&camera.get_id()).unwrap().get_mut(&TypeId::of::<Camera>()).unwrap().as_mut_any().downcast_mut::<Camera>().unwrap();

                        camera_component.begin_camera();
                        Stage::draw_components(
                            Arc::clone(&components),
                            Arc::clone(&resources_clone),
                        );

                        camera_component.end_camera(wait_semaphore,cur_frame);

                    }
                })
            })
            .collect();
    }

    pub fn draw_components(manager:Arc<RwLock<HashMap<u32, HashMap<TypeId, Box<dyn Component>>>>>,resources: Arc<RwLock<ResourcesBundle>>){
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

                    for (id,components) in components_clone.write().unwrap().iter_mut().skip(start).take(end - start) {
                        for (_,component) in components.iter_mut() {
                            component.render(*id, components_clone.write().unwrap().get_mut(id).unwrap(), &resources_clone);
                        }
                    }
                })
            })
            .collect();
    }

    pub fn get_raw_lights(&self) -> Vec<RawLight> {
        let mut raw_lights: Vec<RawLight> = Vec::new();

        let num_cpus = num_cpus::get().max(1);

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let manager_read_lock = Arc::clone(&self.manager.lock().unwrap().borrow_mut().components);
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

    pub fn get_component_manager(&mut self) -> Arc<Mutex<ComponentManager>> {
        Arc::clone(&self.manager)
    }

    pub async fn adfsasd(&self) {
        
    }
}

unsafe impl Send for Stage {}
