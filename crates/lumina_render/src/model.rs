use std::{collections::HashMap, hash::Hash, rc::Rc, sync::Arc};

use ash::vk;

use lumina_atlas::atlas::Atlas;
use lumina_core::{device::Device, texture::Texture, RawLight, Vertex3D};
use lumina_data::descriptor_manager::CurValue;
use lumina_graphic::shader::Shader;
use lumina_object::game_object::{Component, GameObject};
use lumina_pbr::material::Material;
use russimp::scene::{PostProcess, Scene};
use serde_json::Value;

use crate::mesh::{Mesh, Vertex};

pub struct PushConstantData {
    pub model_matrix: glam::Mat4,
    pub normal_matrix: glam::Mat4,
}

pub struct Model {
    device: Arc<Device>,
    pub meshes: Vec<Mesh>,
    pub file_path: String,
    pub materials: Vec<Material>,
    pub mesh_material_bindings: HashMap<usize, usize>,
    pub shader: Shader,
    pub atlas: HashMap<String, Atlas>,
}

impl Model {
    pub fn new_from_array(
        device: Arc<Device>,
        vertex_array: Vec<Vertex>,
        index_array: Vec<u32>,
    ) -> Self {
        let mesh = Mesh::new(Arc::clone(&device), vertex_array, index_array);

        let shader = Shader::new(
            Arc::clone(&device),
            "shaders/default/default_shader.vert",
            "shaders/default/default_shader.frag",
            Vertex3D::setup(),
        );

        let mut mesh_material_bindings: HashMap<usize, usize> = HashMap::new();
        mesh_material_bindings.insert(0, 0);

        let mut atlas = HashMap::new();

        for (id, descriptor_info) in shader.descriptor_manager.descriptor_table.iter() {
            if descriptor_info.value == CurValue::COLOR_IMAGE {
                atlas.insert(id.clone(), Atlas::new());
            }
        }

        Self {
            device: Arc::clone(&device),
            meshes: vec![mesh],
            file_path: String::default(),
            mesh_material_bindings,
            materials: Vec::new(),
            shader,
            atlas,
        }
    }

    pub fn new_from_model(device: Arc<Device>, file_path: &str) -> Self {
        let scene = Scene::from_file(
            file_path,
            vec![
                PostProcess::CalculateTangentSpace,
                PostProcess::Triangulate,
                PostProcess::JoinIdenticalVertices,
                PostProcess::SortByPrimitiveType,
                PostProcess::FlipUVs,
            ],
        )
        .unwrap();

        let mut meshes = Vec::new();

        for mesh in scene.meshes {
            let mut vertex_array: Vec<Vertex> = Vec::new();

            for i in 0..mesh.vertices.len() {
                let position =
                    glam::vec3(mesh.vertices[i].x, mesh.vertices[i].y, mesh.vertices[i].z);
                let normal = glam::vec3(mesh.normals[i].x, mesh.normals[i].y, mesh.normals[i].z);
                let uv = glam::vec2(
                    mesh.texture_coords[0].as_ref().unwrap()[i].x,
                    mesh.texture_coords[0].as_ref().unwrap()[i].y,
                );

                let vertex = Vertex {
                    position,
                    normal,
                    uv,
                };

                vertex_array.push(vertex);
            }

            let mut index_array: Vec<u32> = Vec::new();

            for indices in mesh.faces.iter() {
                for index in &indices.0 {
                    index_array.push(*index);
                }
            }

            let mesh = Mesh::new(Arc::clone(&device), vertex_array, index_array);

            meshes.push(mesh);
        }

        let shader = Shader::new(
            Arc::clone(&device),
            "shaders/default/default_shader.vert",
            "shaders/default/default_shader.frag",
            Vertex3D::setup(),
        );

        let mut atlas = HashMap::new();

        for (id, descriptor_info) in shader.descriptor_manager.descriptor_table.iter() {
            if descriptor_info.value == CurValue::COLOR_IMAGE {
                atlas.insert(id.clone(), Atlas::new());
            }
        }

        Self {
            device: Arc::clone(&device),
            meshes,
            file_path: file_path.to_string(),
            materials: Vec::new(),
            shader,
            mesh_material_bindings: HashMap::new(),
            atlas,
        }
    }

    pub fn render(&mut self, command_buffer: vk::CommandBuffer, device: &Device, frame_index: u32) {
        for (id, mesh) in self.meshes.iter().enumerate() {
            let material = &self.materials[*self.mesh_material_bindings.get(&id).unwrap()];

            self.shader.descriptor_manager.change_buffer_value(
                "MaterialInfo",
                frame_index as u32,
                &[material.get_material_info()],
            );

            unsafe {
                self.device.device().cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.shader.pipeline_layout.unwrap(),
                    0,
                    &[self
                        .shader
                        .descriptor_manager
                        .get_descriptor_set(frame_index as u32)],
                    &[],
                );
            }

            mesh.bind(command_buffer, device);
            mesh.draw(command_buffer, device);
        }
    }

    pub fn raw_render(&self, command_buffer: vk::CommandBuffer, device: &Device) {
        for (id, mesh) in self.meshes.iter().enumerate() {
            mesh.bind(command_buffer, device);
            mesh.draw(command_buffer, device);
        }
    }

    pub fn add_texture(&mut self, label: &str, textures: Vec<&mut Texture>) {
        if self.atlas.contains_key(label) {
            self.atlas.get_mut(label).unwrap().pack_textures(textures);
        } else {
            eprintln!("ERROR: Failed to get the desired texture atlas!")
        }
    }
}

/*impl GameObjectTrait for Model {
    fn render(
        &self,
        device: &Device,
        game_object: &GameObject,
        command_buffer: vk::CommandBuffer,
    ) {
        let _push = PushConstantData {
            model_matrix: game_object.transform.get_mat4(),
            normal_matrix: game_object.transform.get_normal_matrix(),
        };

        for mesh in &self.meshes {
            mesh.bind(command_buffer, device);
            mesh.draw(command_buffer, device);
        }
    }

    fn game_object(&self) -> &GameObject {
        return &self.game_object;
    }
}*/

impl Drop for Model {
    fn drop(&mut self) {
        self.shader.destroy(&self.device);
        for mesh in self.meshes.iter_mut() {
            drop(mesh);
        }
    }
}

impl Component for Model {
    fn convert_to_json(&self,id:u32) -> Value {
        let mut json = serde_json::json!({
            "id": id,
            "file": self.file_path,
            "materials": [],
            "meshes": [],
        });

        for (mat_id, material) in self.materials.iter().enumerate() {
            let mut id = 0;

            for (m_id, m_key) in self.mesh_material_bindings.iter() {
                if *m_key == mat_id {
                    id = *m_id;
                }
            }

            json["materials"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({
                    "parent_id": id,
                    "ambient": material.ambient.to_array(),
                    "ambient_texture": material.ambient_texture.get_new_path(),
                    "diffuse": material.diffuse.to_array(),
                    "metallic": material.metallic.to_array(),
                    "metallic_texture": material.metallic_texture.get_new_path()
                }));
        }

        for (id, mesh) in self.meshes.iter().enumerate() {
            json["meshes"]
                .as_array_mut()
                .unwrap()
                .push(mesh.to_json(id));
        }

        return json;
    }
}

unsafe impl Send for Model {}

unsafe impl Sync for Model {}
