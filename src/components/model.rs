use ash::vk;

use crate::{
    engine::device::Device,
    graphics::mesh::{Mesh, Vertex},
};

use super::game_object::{GameObject, GameObjectTrait};

struct PushConstantData {
    model_matrix: glam::Mat4,
    normal_matrix: glam::Mat4,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub game_object: GameObject,
}

impl Model {
    pub fn new() -> Self {
        let game_obj = GameObject::create_game_object();
        Self {
            meshes: Vec::new(),
            game_object: game_obj,
        }
    }

    pub fn create_mesh_from_array(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        device: &Device,
    ) {
        self.meshes.push(Mesh::new(device, vertices, indices));
    }

    pub fn load_model(_filepath: String) {}

}

impl GameObjectTrait for Model {
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
}
