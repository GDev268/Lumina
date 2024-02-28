use std::{
    any::TypeId,
    borrow::BorrowMut,
    collections::HashMap,
    fs::File,
    io::Read,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Barrier, Mutex, RwLock},
    thread::{self, JoinHandle},
};

use ash::vk;
use async_std::path;
use lumina_core::{
    device::Device, framebuffer::Framebuffer, image::Image, texture::Texture, window::Window,
    Vertex3D,
};
use lumina_data::{buffer::Buffer, descriptor_manager::DescriptorManager};
use lumina_files::{
    loader::Loader,
    saver::{LuminaFile, LuminaFileType, Saver},
};
use lumina_graphic::shader::Shader;
use lumina_object::{
    game_object::{Component, GameObject},
    transform::Transform,
};
use lumina_path::PATHS;
use lumina_pbr::light::Light;
use lumina_render::{
    camera::Camera, mesh::Mesh, model::Model, model::PushConstantData, renderer::Renderer,
};
use serde_json::Value;

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
    pub name: String,
    pub manager: Query,
}

impl Stage {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            manager: Query::new(),
        }
    }

    pub fn render(
        &mut self,
        renderer: Arc<RwLock<Renderer>>,
        device: Arc<Device>,
        command_buffer: vk::CommandBuffer,
        camera: Camera,
    ) {
        let num_cpus = num_cpus::get();

        let frame_index = renderer.read().unwrap().get_frame_index();

        /*let raw_light_3: LightInfo = LightInfo {
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

        let raw_light_2 = RawLight {
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
        };*/

        let raw_light: lumina_core::RawLight = lumina_core::RawLight {
            position: [0.0, -5.0, 5.0],
            rotation: [-0.6, 90.0, -5.9],
            color: [0.0, 1.0, 1.0],
            intensity: 10.0,
            spot_size: 7.5,
            linear: 0.7,
            quadratic: 1.8,
            light_type: 2,
            _padding1: 0,
            _padding2: 0,
        };

        let raw_lights = self.get_raw_lights();

        for (_, entity) in self.manager.entities.write().unwrap().iter_mut() {
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
                    let push = PushConstantData {
                        model_matrix,
                        normal_matrix,
                    };

                    cube.render(
                        command_buffer,
                        &device,
                        frame_index as u32,
                        push,
                        camera.get_matrix(),
                        raw_lights.clone(),
                        camera.get_position().to_array(),
                    );
                }
            }
        }
    }

    pub fn save_scene(&self) {
        let num_cpus = num_cpus::get();

        let saver = Arc::new(RwLock::new(Saver::new()));

        let mut light_count: u32 = 0;

        saver.write().unwrap().modify_project_name(&self.name);

        for (id, entity) in self.manager.entities.read().unwrap().iter() {
            let mut saver_lock = saver.write().unwrap();
            saver_lock.json["game_objects"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!(*id));

            let is_light = entity.read().unwrap().has_component::<Light>();
            let is_model: bool = entity.read().unwrap().has_component::<Model>();
            let is_transform: bool = entity.read().unwrap().has_component::<Transform>();

            if is_light {
                if let Some(light) = entity.read().unwrap().get_component::<Light>() {
                    saver_lock.json["lights"]
                        .as_array_mut()
                        .unwrap()
                        .push(light.convert_to_json(*id));
                }

                light_count += 1;
            }

            if is_model {
                if let Some(model) = entity.read().unwrap().get_component::<Model>() {
                    saver_lock.json["models"]
                        .as_array_mut()
                        .unwrap()
                        .push(model.convert_to_json(*id));
                }
            }

            if is_transform {
                if let Some(transform) = entity.read().unwrap().get_component::<Transform>() {
                    saver_lock.json["transforms"]
                        .as_array_mut()
                        .unwrap()
                        .push(transform.convert_to_json(*id));
                }
            }
        }

        saver.write().unwrap().json["light_count"] = serde_json::json!(light_count);

        let paths = Arc::new(unsafe { PATHS.clone() });

        saver.write().unwrap().create_directory("textures");

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let paths_clone = paths.clone();

                let saver_clone = saver.clone();

                thread::spawn(move || {
                    let size = paths_clone.len();
                    let start = i * size / num_cpus;
                    let end = if i == num_cpus - 1 {
                        size
                    } else {
                        (i + 1) * size / num_cpus
                    };

                    for (raw_path, new_path) in paths_clone.iter().skip(start).take(end - start) {
                        let mut file = File::open(raw_path).unwrap();
                        let mut contents = Vec::new();
                        file.read_to_end(&mut contents).unwrap();

                        let file =
                            LuminaFile::new(LuminaFileType::Jpg, new_path.to_string(), contents);
                        saver_clone
                            .write()
                            .unwrap()
                            .insert_file_into_directory("textures", file)
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }


        saver.write().unwrap().save_data();
    }

    pub fn load_scene(
        &mut self,
        device: Arc<Device>,
        render_pass: vk::RenderPass,
        file_path: &str,
    ) {
        let num_cpus = num_cpus::get();

        let mut loader = Arc::new(RwLock::new(Loader::new()));

        loader.write().unwrap().load_file(file_path.to_string());

        let file_content = loader
            .read()
            .unwrap()
            .directories
            .get("gameData")
            .unwrap()
            .files
            .iter()
            .find(|file| "scene.json" == file.file_name)
            .unwrap()
            .file_content
            .clone();

        let json_string = String::from_utf8(file_content).unwrap();
        let mut json: Value = serde_json::from_str(&json_string).unwrap();
        //panic!("{:?}",serde_json::to_string_pretty(&json));

        let light_count = json["light_count"].as_u64().unwrap_or(0) as u32;

        let mut game_objects = HashMap::new();

        for game_object_id in json["game_objects"].as_array().unwrap().iter() {
            let game_object = self
                .manager
                .spawn_with_id(game_object_id.as_u64().unwrap() as u32);

            game_objects.insert(game_object_id.as_u64().unwrap() as u32, game_object);
        }

        for transform_json in json["transforms"].as_array().unwrap().iter() {
            let transform = Transform {
                translation: glam::vec3(
                    transform_json["transform"].as_array().unwrap()[0]
                        .as_f64()
                        .unwrap() as f32,
                    transform_json["transform"].as_array().unwrap()[1]
                        .as_f64()
                        .unwrap() as f32,
                    transform_json["transform"].as_array().unwrap()[2]
                        .as_f64()
                        .unwrap() as f32,
                ),
                rotation: glam::vec3(
                    transform_json["rotation"].as_array().unwrap()[0]
                        .as_f64()
                        .unwrap() as f32,
                    transform_json["rotation"].as_array().unwrap()[1]
                        .as_f64()
                        .unwrap() as f32,
                    transform_json["rotation"].as_array().unwrap()[2]
                        .as_f64()
                        .unwrap() as f32,
                ),
                scale: glam::vec3(
                    transform_json["scale"].as_array().unwrap()[0]
                        .as_f64()
                        .unwrap() as f32,
                    transform_json["scale"].as_array().unwrap()[1]
                        .as_f64()
                        .unwrap() as f32,
                    transform_json["scale"].as_array().unwrap()[2]
                        .as_f64()
                        .unwrap() as f32,
                ),
            };

            let game_object = game_objects
                .get(&(transform_json["id"].as_u64().unwrap() as u32))
                .unwrap();

            self.manager.push(game_object, transform);
        }

        for model_json in json["models"].as_array().unwrap().iter() {
            let mut model = Model::new(Arc::clone(&device));

            let mut meshes: Vec<Mesh> = Vec::new();
            let mut materials: Vec<lumina_pbr::material::Material> = Vec::new();

            for mesh in model_json["meshes"].as_array().unwrap().iter() {
                let mut vertices = Vec::new();
                let mut indices = Vec::new();

                for vertex in mesh["vertices"].as_array().unwrap().iter() {
                    vertices.push(Vertex3D {
                        position: glam::vec3(
                            vertex["position"].as_array().unwrap()[0].as_f64().unwrap() as f32,
                            vertex["position"].as_array().unwrap()[1].as_f64().unwrap() as f32,
                            vertex["position"].as_array().unwrap()[2].as_f64().unwrap() as f32,
                        ),
                        normal: glam::vec3(
                            vertex["normal"].as_array().unwrap()[0].as_f64().unwrap() as f32,
                            vertex["normal"].as_array().unwrap()[1].as_f64().unwrap() as f32,
                            vertex["normal"].as_array().unwrap()[2].as_f64().unwrap() as f32,
                        ),
                        uv: glam::vec2(
                            vertex["uv"].as_array().unwrap()[0].as_f64().unwrap() as f32,
                            vertex["uv"].as_array().unwrap()[1].as_f64().unwrap() as f32,
                        ),
                    });
                }

                for indice in mesh["indices"].as_array().unwrap().iter() {
                    indices.push(indice.as_u64().unwrap() as u32)
                }

                meshes.push(Mesh::new(Arc::clone(&device), vertices, indices));
            }

            for material_json in model_json["materials"].as_array().unwrap().iter() {
                let mut material = lumina_pbr::material::Material::new(
                    glam::vec3(
                        material_json["ambient"].as_array().unwrap()[0]
                            .as_f64()
                            .unwrap() as f32,
                        material_json["ambient"].as_array().unwrap()[1]
                            .as_f64()
                            .unwrap() as f32,
                        material_json["ambient"].as_array().unwrap()[2]
                            .as_f64()
                            .unwrap() as f32,
                    ),
                    glam::vec3(
                        material_json["diffuse"].as_array().unwrap()[0]
                            .as_f64()
                            .unwrap() as f32,
                        material_json["diffuse"].as_array().unwrap()[1]
                            .as_f64()
                            .unwrap() as f32,
                        material_json["diffuse"].as_array().unwrap()[2]
                            .as_f64()
                            .unwrap() as f32,
                    ),
                    glam::vec3(
                        material_json["metallic"].as_array().unwrap()[0]
                            .as_f64()
                            .unwrap() as f32,
                        material_json["metallic"].as_array().unwrap()[1]
                            .as_f64()
                            .unwrap() as f32,
                        material_json["metallic"].as_array().unwrap()[2]
                            .as_f64()
                            .unwrap() as f32,
                    ),
                    1.0,
                );

                material.ambient_texture =
                    Texture::new_raw(material_json["ambient_texture"].as_str().unwrap());
                material.metallic_texture =
                    Texture::new_raw(material_json["metallic_texture"].as_str().unwrap());

                materials.push(material);
            }

            model.meshes = meshes;
            model.materials = materials;
            
            model.init_model(render_pass, light_count as u64);

            let game_object = game_objects
                .get(&(model_json["id"].as_u64().unwrap() as u32))
                .unwrap();

            self.manager.push(game_object, model);
        }

        for light_json in json["lights"].as_array().unwrap().iter() {
            let mut light = Light::new();

            light.change_light_type(light_json["light_type"].as_u64().unwrap() as u32);
            light.change_color(glam::vec3(
                light_json["color"].as_array().unwrap()[0].as_f64().unwrap() as f32,
                light_json["color"].as_array().unwrap()[1].as_f64().unwrap() as f32,
                light_json["color"].as_array().unwrap()[2].as_f64().unwrap() as f32,
            ));
            light.change_intensity(light_json["intensity"].as_f64().unwrap() as f32);
            light.change_range(light_json["range"].as_f64().unwrap() as f32);
            light.change_spot_size(light_json["spot_size"].as_f64().unwrap() as f32);

            let game_object = game_objects
                .get(&(light_json["id"].as_u64().unwrap() as u32))
                .unwrap();

            self.manager.push(game_object, light);
        }

        println!("{:?}", self.manager.entities);
    }


    pub fn get_raw_lights(&mut self) -> Vec<lumina_core::RawLight> {
        let num_cpus = num_cpus::get();

        let mut raw_lights = Vec::new();


        let handles: Vec<_> = (0..num_cpus)
        .map(|i| {
            let manager = self.manager.entities.clone();

            thread::spawn(move || {
                let size = manager.read().unwrap().len();
                let start = i * size / num_cpus;
                let end = if i == num_cpus - 1 {
                    size
                } else {
                    (i + 1) * size / num_cpus
                };

                let mut raw_lights = Vec::new();


                for (id,entity) in manager.read().unwrap().iter().skip(start).take(end - start) {
                    let transform = entity.read().unwrap().get_component::<Transform>().unwrap().clone();

                    if let Some(light) = entity.read().unwrap().get_component::<Light>() {
                        raw_lights.push(light.create_raw_light(id, &transform))
                    }
                }

                raw_lights
            })
        }).collect();

        for handle in handles {
            raw_lights.extend(handle.join().unwrap())
        }

        raw_lights
    }
}

unsafe impl Send for Stage {}
