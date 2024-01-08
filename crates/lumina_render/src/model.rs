use std::rc::Rc;

use lumina_bundle::ResourcesBundle;
use lumina_core::{device::Device, Vertex3D};
use lumina_graphic::shader::Shader;
use lumina_object::{game_object::Component, delete_component_id, create_component_id};

use crate::mesh::Mesh;

struct Model {
    device: Rc<Device>,
    mesh: Mesh,
    shader: Shader,
    component_id:u32
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
            component_id: create_component_id()
        }
    }
}

impl Component for Model {
    fn get_id(&self) -> u32{
       self.component_id
    }

    fn clone(&self) -> Box<dyn Component> {
        let model = Model{
            device: Rc::clone(&self.device),
            mesh: self.mesh.clone(),
            shader: self.shader.clone(),
            component_id: self.component_id
        };

        Box::new(model)
    }

    fn update(&mut self) {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

unsafe impl Send for Model {}

unsafe impl Sync for Model {}

impl Drop for Model {
    fn drop(&mut self) {
        delete_component_id(self.component_id)
    }
}