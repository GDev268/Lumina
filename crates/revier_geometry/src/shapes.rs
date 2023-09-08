use revier_core::device::Device;
use revier_object::game_object::GameObject;
use revier_render::mesh::Vertex;
use revier_scene::scene::Scene;

use crate::model::Model;

pub fn cube(scene: &mut Scene, device: &Device) -> GameObject {
    let mut model = Model::new();

    let vertices: Vec<Vertex> = vec![
        // left face (white)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.9, 0.9),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // right face (yellow)
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.8, 0.1),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // top face (orange)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.9, 0.6, 0.1),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // bottom face (red)
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.8, 0.1, 0.1),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // nose face (blue)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            color: glam::Vec3::new(0.1, 0.1, 0.8),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // tail face (green)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            color: glam::Vec3::new(0.1, 0.8, 0.1),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
    ];

    model.create_mesh_from_array(vertices, Vec::new(), device);

    let game_object = scene.spawn();
    scene.push(&game_object,model);
        
    return game_object;
}
