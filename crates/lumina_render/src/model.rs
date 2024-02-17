use std::{rc::Rc, sync::Arc};

use ash::vk;

use lumina_core::device::Device;
use lumina_graphic::shader::Shader;
use lumina_object::game_object::{Component, GameObject};
use russimp::scene::{PostProcess, Scene};

use crate::mesh::{Mesh, Vertex};

pub struct PushConstantData {
    pub model_matrix: glam::Mat4,
    pub normal_matrix: glam::Mat4,
}

pub struct Model {
    device: Arc<Device>,
    pub meshes: Vec<Mesh>,
    pub shader: Shader,
    component_id: u32,
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
            "shaders/default_shader.vert",
            "shaders/default_shader.frag",
        );

        Self {
            device: Arc::clone(&device),
            meshes: vec![/*mesh*/],
            shader,
            component_id: 0,
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
                let position = glam::vec3(
                    mesh.vertices[i].x,
                    mesh.vertices[i].y,
                    mesh.vertices[i].z,
                );
                let normal = glam::vec3(
                    mesh.normals[i].x,
                    mesh.normals[i].y,
                    mesh.normals[i].z,
                );
                let uv = glam::vec2(mesh.texture_coords[0].as_ref().unwrap()[i].x, mesh.texture_coords[0].as_ref().unwrap()[i].y);
    
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
            shader,
            component_id: 0,
        }
    }

    pub fn render(&self,command_buffer:vk::CommandBuffer,device: &Device) {
        for mesh in self.meshes.iter() {
            mesh.bind(command_buffer, device);
            mesh.draw(command_buffer, device);
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

impl Component for Model {}

unsafe impl Send for Model {}

unsafe impl Sync for Model {}