use std::{collections::HashMap, hash::Hash, rc::Rc, sync::Arc};

use ash::vk;

use lumina_core::{device::Device, RawLight};
use lumina_graphic::shader::Shader;
use lumina_object::game_object::{Component, GameObject};
use lumina_pbr::material::Material;
use russimp::scene::{PostProcess, Scene};
use serde_json::Value;

use crate::mesh::{Mesh, Vertex};


pub struct Canvas {
    device: Arc<Device>,
    pub meshes: Vec<Mesh>,
    pub file_path: String,
    pub materials: Vec<Material>,
    pub mesh_material_bindings: HashMap<usize, usize>,
    pub shader: Shader,
}

impl Canvas {
    pub fn new_from_array(
        device: Arc<Device>,
        vertex_array: Vec<Vertex>,
        index_array: Vec<u32>,
    ) -> Self {
        let mesh = Mesh::new(Arc::clone(&device), vertex_array, index_array);

        let shader = Shader::new(
            Arc::clone(&device),
            "shaders/default_shader.vert",
            "shaders/default_shader.frag",
        );

        let mut mesh_material_bindings: HashMap<usize, usize> = HashMap::new();
        mesh_material_bindings.insert(0, 0);

        Self {
            device: Arc::clone(&device),
            meshes: vec![mesh],
            file_path: String::default(),
            mesh_material_bindings,
            materials: Vec::new(),
            shader,
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
            "shaders/default_shader.vert",
            "shaders/default_shader.frag",
        );

        Self {
            device: Arc::clone(&device),
            meshes,
            file_path: file_path.to_string(),
            materials: Vec::new(),
            shader,
            mesh_material_bindings: HashMap::new(),
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
}
