/*use lumina_core::device::Device;
use lumina_object::game_object::GameObject;
use lumina_render::mesh::Vertex;
use lumina_scene::query::Query;

use crate::model::Model;

pub fn cube(scene: &mut Query, device: &Device) -> GameObject {
    let mut model = Model::new();

    let vertices: Vec<Vertex> = vec![
        // left face (white)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // right face (yellow)
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // top face (orange)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // bottom face (red)
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // nose face (blue)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // tail face (green)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
    ];

    model.create_mesh_from_array(vertices, Vec::new(), device);

    let game_object = scene.spawn();
    scene.push(&game_object,model);
        
    return game_object;
}

pub fn mono_cube(scene: &mut Query, device: &Device) -> GameObject {
    let mut model = Model::new();

    let vertices: Vec<Vertex> = vec![
        // left face (white)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // right face (yellow)
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(1.0, 0.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // top face (orange)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, -1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // bottom face (red)
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 1.0, 0.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // nose face (blue)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5),
            normal: glam::Vec3::new(0.0, 0.0, 1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        // tail face (green)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5),
            normal: glam::Vec3::new(0.0, 0.0, -1.0),
            uv: glam::Vec2::new(1.0, 1.0),
        },
    ];

    model.create_mesh_from_array(vertices, Vec::new(), device);

    let game_object = scene.spawn();
    scene.push(&game_object,model);
        
    return game_object;
}*/

