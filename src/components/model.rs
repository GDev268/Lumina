use crate::{graphics::{mesh::{Mesh, Vertex}, shader::Shader}, engine::device::Device};

use super::game_object::GameObject;

struct PushConstantData {
    model_matrix: glam::Mat4,
    normal_matrix: glam::Mat4,
}

struct Model{
    meshes:Vec<Mesh>,
    game_object:GameObject
}

impl Model{
    pub fn new() -> Self {
        let game_obj = GameObject::create_game_object();
        Self { meshes: Vec::new(), game_object: game_obj }
    }

    pub fn create_mesh_from_array(vertices:&Vec<f32>,num_vertices:usize,indices:Vec<u32>,device:&Device) -> Mesh{

        let mut mesh_vertices:Vec<Vertex> = Vec::new();
        let stride = std::mem::size_of::<Vertex>() / std::mem::size_of::<f32>();
        for i in 0..num_vertices{

            let position: glam::Vec3 = glam::vec3(vertices[i * stride + 0], vertices[i * stride + 1], vertices[i * stride + 2]);
            let uv =  glam::vec2(vertices[i * stride + 3], vertices[i * stride + 4]);
            let normal =  glam::vec3(vertices[i * stride + 5], vertices[i * stride + 6], vertices[i * stride + 7]);
            let color = glam::vec3(1.0, 1.0, 1.0);

            mesh_vertices.push(Vertex{position,color,normal,uv});
        }


        return Mesh::new(device,mesh_vertices,indices);
    }

    pub fn render(&self,device:&Device,shader:Shader){
        let push = PushConstantData{
            model_matrix: self.game_object.transform.get_mat4(),
            normal_matrix: self.game_object.transform.get_normal_matrix()
        };
    }

    pub fn load_model(filepath:String){
        
    }
}
