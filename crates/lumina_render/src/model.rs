use std::rc::Rc;

use ash::vk;

use lumina_core::device::Device;
use lumina_render::mesh::{Mesh, Vertex};
use lumina_object::game_object::{GameObject,Component};

pub struct PushConstantData {
    pub model_matrix: glam::Mat4,
    pub normal_matrix: glam::Mat4,
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
        device: Rc<Device>,
    ) {
        self.meshes.push(Mesh::new(device, vertices, indices));
    }

    pub fn load_model(_filepath: String) {}

    pub fn render(
        &self,
        device: &Device,
        command_buffer: vk::CommandBuffer,
    ) {
        self.meshes[0].bind(command_buffer, device);
        self.meshes[0].draw(command_buffer, device);

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
