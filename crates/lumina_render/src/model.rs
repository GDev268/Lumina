use std::{
    any::TypeId,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

use ash::vk;
use lumina_bundle::ResourcesBundle;
use lumina_core::{device::Device, Vertex3D};
use lumina_graphic::{pipeline::PipelineConfiguration, shader::Shader};
use lumina_object::{
    create_component_id, delete_component_id, game_object::Component, transform::Transform,
};

use crate::mesh::Mesh;

pub struct PushConstantData {
    pub model_matrix: glam::Mat4,
    pub normal_matrix: glam::Mat4,
}

pub struct Model {
    device: Rc<Device>,
    mesh: Mesh,
    shader: Shader,
    component_id: u32,
}

impl Model {
    pub fn new_from_array(
        device: Rc<Device>,
        vertex_array: Vec<Vertex3D>,
        index_array: Vec<u32>,
    ) -> Self {
        let mesh = Mesh::new(Rc::clone(&device), vertex_array, index_array);
        let mut shader = Shader::new(
            Rc::clone(&device),
            "shaders/default/default_shader.vert",
            "shaders/default/default_shader.frag",
        );

        Self {
            device: Rc::clone(&device),
            mesh,
            shader,
            component_id: create_component_id(),
        }
    }
}

impl Component for Model {
    fn get_id(&self) -> u32 {
        self.component_id
    }

    fn clone(&self) -> Box<dyn Component> {
        let model = Model {
            device: Rc::clone(&self.device),
            mesh: self.mesh.clone(),
            shader: self.shader.clone(),
            component_id: self.component_id,
        };

        Box::new(model)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn update(
        &mut self,
        id: u32,
        component: Arc<RwLock<HashMap<u32, HashMap<TypeId, Box<dyn Component>>>>>,
        resources_bundle: &Arc<RwLock<ResourcesBundle>>,
    ) {
        let binding = component.read().unwrap();
        let component_group = binding.get(&id).unwrap();

        //self.shader.descriptor_manager.change_buffer_value("LightInfo".to, resources_bundle.read().unwrap().cur_frame, &resources_bundle.raw_lights);

        for (type_id,component) in component_group {

        }

        drop(component_group);
        drop(binding);
        drop(resources_bundle);
    }

    fn render(
        &mut self,
        id: u32,
        component: Arc<RwLock<HashMap<u32, HashMap<TypeId, Box<dyn Component>>>>>,
        resources_bundle: Arc<RwLock<ResourcesBundle>>
    ) {
        panic!("a");

        let binding = component.read().unwrap();
        let components = binding.get(&self.component_id).unwrap();

        let mut pipeline_config = PipelineConfiguration::default();
        pipeline_config.attribute_descriptions = self.mesh.get_attribute_descriptions().clone();
        pipeline_config.binding_descriptions = self.mesh.get_binding_descriptions().clone();

        self.shader.create_pipeline_layout(true);
        self.shader
            .create_pipeline(resources_bundle.read().unwrap().cur_render_pass, pipeline_config);

        self.shader.descriptor_manager.change_buffer_value(
            "ProjectionViewMatrix",
            resources_bundle.read().unwrap().cur_frame,
            &[resources_bundle.read().unwrap().cur_projection],
        );

        let transform = components
            .get(&TypeId::of::<Transform>())
            .unwrap()
            .as_any()
            .downcast_ref::<Transform>()
            .unwrap();

        let constant_data = PushConstantData {
            model_matrix: transform.get_mat4(),
            normal_matrix: transform.get_normal_matrix(),
        };

        let push_bytes: &[u8] = unsafe {
            let struct_ptr = &constant_data as *const _ as *const u8;
            std::slice::from_raw_parts(struct_ptr, std::mem::size_of::<PushConstantData>())
        };

        unsafe {
            self.device.device().cmd_push_constants(
                resources_bundle.read().unwrap().command_buffer,
                self.shader.pipeline_layout.unwrap(),
                vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                0,
                push_bytes,
            );
        }

        self.mesh.bind(resources_bundle.read().unwrap().command_buffer, &self.device);
        self.mesh.draw(resources_bundle.read().unwrap().command_buffer, &self.device);
    }
}

unsafe impl Send for Model {}

unsafe impl Sync for Model {}

impl Drop for Model {
    fn drop(&mut self) {
        delete_component_id(self.component_id)
    }
}
