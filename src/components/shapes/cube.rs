use crate::{
    components::game_object::GameObjectTrait,
    engine::device::Device,
    graphics::{
        mesh::{Mesh, Vertex},
    },
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
        let data: [f32; 70] = [
            1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 0.6, 0.5, 0.9, 0.5, 0.9, 0.8, 0.6, 0.8,
            0.4, 0.8, 0.6, 1.0, 0.4, 1.0, 0.4, 0.0, 0.6, 0.0, 0.6, 0.2, 0.4, 0.2, 0.1, 0.5, 0.4,
            0.5, 0.1, 0.8, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, -1.0,
        ];

        let indices: [u32; 24] = [
            1, 5, 7, 3, 4, 3, 7, 8, 8, 7, 5, 6, 6, 2, 4, 8, 2, 1, 3, 4, 6, 5, 1, 2,
        ];
        let meshes: Vec<Mesh> = vec![Cube::create_mesh_from_array(device, &data, &indices, 8)];
        drop(data);
        drop(indices);
        Self {
            meshes,
            game_object,
        }
    }

    pub fn create_mesh_from_array(
        device: &Device,
        vertices: &[f32; 70],
        indices: &[u32; 24],
        num_vertices: usize,
    ) -> Mesh {
        let mut mesh_vertices: Vec<Vertex> = Vec::new();
        let stride = (std::mem::size_of::<glam::Vec3>() * 2) / std::mem::size_of::<f32>();
        for i in 0..num_vertices {
            let position: glam::Vec3 = glam::vec3(
                vertices[i * stride + 0],
                vertices[i * stride + 1],
                vertices[i * stride + 2],
            );
            let uv = glam::vec2(vertices[i * stride + 3],vertices[i * stride + 4]);
            let normal = glam::vec3(
                vertices[i * stride + 5],
                vertices[i * stride + 6],
                vertices[i * stride + 7],
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
}

impl GameObjectTrait for Cube {
    fn render(&self, _device: &Device, game_object: &GameObject) {
        let _push = PushConstantData {
            model_matrix: game_object.transform.get_mat4(),
            normal_matrix: game_object.transform.get_normal_matrix(),
        };

        println!("{}", game_object.transform.get_mat4());
    }

    fn game_object(&self) -> &GameObject {
        return &self.game_object;
    }
}
