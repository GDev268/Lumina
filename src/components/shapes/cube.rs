use crate::{
    engine::device::Device,
    graphics::{
        mesh::{Mesh, Vertex},
        shader::Shader,
    }, components::game_object::GameObjectTrait,
};

use crate::components::game_object::GameObject;

pub struct PushConstantData {
    pub model_matrix: glam::Mat4,
    pub normal_matrix: glam::Mat4,
}

pub struct Cube {
    meshes: Vec<Mesh>,
    pub game_object: GameObject,
}

impl Cube {
    pub fn new(device: &Device) -> Self {
        let game_object = GameObject::create_game_object();
        let data: [f32; 64] = [
            1.0, -1.0, 1.0, 0.9, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.6, 0.8, 0.0, 0.0, 1.0, 1.0,
            -1.0, -1.0, 0.6, 0.5, 0.0, 1.0, 0.0, 1.0, 1.0, -1.0, 0.4, 1.0, 1.0, 0.5, 0.0, -1.0,
            -1.0, 1.0, 0.4, 0.8, 1.0, 0.0, 0.0, -1.0, 1.0, 1.0, 0.6, 0.0, 1.0, 0.0, 1.0, -1.0,
            -1.0, -1.0, 0.4, 0.2, 0.0, 1.0, 0.0, -1.0, 1.0, -1.0, 0.4, 0.0, 1.0, 0.5, 0.0,
        ];

        let indices: [u32; 36] = [
            5, 3, 1, 3, 8, 4, 7, 6, 8, 2, 8, 6, 1, 4, 2, 5, 2, 6, 5, 7, 3, 3, 7, 8, 7, 5, 6, 2, 4,
            8, 1, 3, 4, 5, 1, 2,
        ];
        let meshes: Vec<Mesh> = Vec::new();   /*vec![Cube::create_mesh_from_array(device, &data, &indices, 28)];
        drop(data);
        drop(indices);*/
        Self {
            meshes,
            game_object,
        }
    }

    pub fn create_mesh_from_array(
        device: &Device,
        vertices: &[f32; 64],
        indices: &[u32; 36],
        num_vertices: usize,
    ) -> Mesh {
        let mut mesh_vertices: Vec<Vertex> = Vec::new();
        let stride = (std::mem::size_of::<glam::Vec3>() * 2) / std::mem::size_of::<f32>();
        for i in 0..num_vertices {
            println!("{}", i * stride + 0);
            let position: glam::Vec3 = glam::vec3(
                vertices[i * stride + 0],
                vertices[i * stride + 1],
                vertices[i * stride + 2],
            );
            let uv = glam::vec2(vertices[i * stride + 3], vertices[i * stride + 4]);
            let normal = glam::vec3(
                vertices[i * stride + 3],
                vertices[i * stride + 4],
                vertices[i * stride + 5],
            );
            let color = glam::vec3(1.0, 1.0, 1.0);

            mesh_vertices.push(Vertex {
                position,
                color,
                normal,
                uv,
            });
        }

        return Mesh::new(device, mesh_vertices, indices.to_vec());
    }

    pub fn render(&self, device: &Device, shader: Shader) {
        let push = PushConstantData {
            model_matrix: self.game_object.transform.get_mat4(),
            normal_matrix: self.game_object.transform.get_normal_matrix(),
        };
    }
}

impl GameObjectTrait for Cube{
    fn render(&mut self,device:&Device,game_object:&GameObject){
        let push = PushConstantData{
            model_matrix: self.game_object.transform.get_mat4(),
            normal_matrix: self.game_object.transform.get_normal_matrix()
        };


    }

    fn game_object(&self) -> &GameObject{
        return &self.game_object;
    }
}
