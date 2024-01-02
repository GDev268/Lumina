use std::rc::Rc;

use lumina_bundle::ResourcesBundle;
use lumina_core::{device::Device, Vertex3D};
use lumina_graphic::shader::Shader;
use lumina_object::game_object::Component;

use crate::mesh::Mesh;

struct Model {
    device: Rc<Device>,
    mesh: Mesh,
    shader: Shader,
}

impl Model {
    pub fn new_from_array(
        device: Rc<Device>,
        vertex_array: Vec<Vertex3D>,
        index_array: Vec<u32>,
    ) -> Self {
        let mesh = Mesh::new(Rc::clone(&device), vertex_array, index_array);
        let shader = Shader::new(
            Rc::clone(&device),
            "shaders/default/default_shader.vert",
            "shaders/default/default_shader.frag",
        );

        Self {
            device: Rc::clone(&device),
            mesh,
            shader,
        }
    }
}

impl Component for Model {}
