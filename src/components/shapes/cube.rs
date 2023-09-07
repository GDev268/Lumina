use ash::vk;

use crate::{
    engine::device::Device,
    graphics::{
        mesh::{Mesh, Vertex},
    },
};

use crate::components::game_object::GameObject;

#[derive(Debug)]
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
        let data: [f32; 240] = [
            // Front face
            -0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // Vertex 0
            0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, // Vertex 1
            0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, // Vertex 2
            -0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // Vertex 3
        
            // Back face
            -0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, // Vertex 4
            0.5, -0.5, -0.5, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, // Vertex 5
            0.5, 0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, // Vertex 6
            -0.5, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, // Vertex 7
        
            // Top face
            -0.5, 0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // Vertex 8
            0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, // Vertex 9
            0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, // Vertex 10
            -0.5, 0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // Vertex 11
        
            // Bottom face
            -0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // Vertex 12
            0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, // Vertex 13
            0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, // Vertex 14
            -0.5, -0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // Vertex 15
        
            // Left face
            -0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // Vertex 16
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, // Vertex 17
            -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, // Vertex 18
            -0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // Vertex 19
        
            // Right face
            0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, // Vertex 20
            0.5, 0.5, -0.5, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, // Vertex 21
            0.5, -0.5, 0.5, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, // Vertex 22
            0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, // Vertex 23
        ];
        

        let indices: [u32; 36] = [
            0, 1, 2, 2, 3, 0,   // Front face
            4, 5, 6, 6, 7, 4,   // Back face
            8, 9, 10, 10, 11, 8, // Top face
            12, 13, 14, 14, 15, 12, // Bottom face
            16, 17, 18, 18, 19, 16, // Left face
            20, 21, 22, 22, 23, 20, // Right face
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
        vertices: &[f32; 240],
        indices: &[u32; 36],
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

    pub fn test_render(&mut self,command_buffer:vk::CommandBuffer,device:&Device){
        self.meshes[0].bind(command_buffer, device);
        self.meshes[0].draw(command_buffer, device);
    }
}

/*impl GameObjectTrait for Cube {
    fn render(&self, device: &Device, game_object: &GameObject,command_buffer:vk::CommandBuffer) {
        
        for mesh in &self.meshes{
            mesh.bind(command_buffer, device);
            mesh.draw(command_buffer,device);
        }

    }

    fn game_object(&self) -> &GameObject {
        return &self.game_object;
    }
}*/
