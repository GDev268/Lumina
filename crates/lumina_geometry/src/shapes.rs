use std::{rc::Rc, sync::Arc};

use lumina_core::device::Device;
use lumina_object::game_object::GameObject;
use lumina_render::{mesh::Vertex, model::Model};
/*use lumina_scene::query::Query;

pub fn cube(scene: &mut Query, device: Arc<Device>) -> GameObject {
    let vertices: Vec<Vertex> = vec![
        // left face (white)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // right face (yellow)
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // top face (orange)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // bottom face (red)
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // nose face (blue)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // tail face (green)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
    ];

    let model = Model::new_from_array(Arc::clone(&device),vertices,Vec::new());

    let game_object = scene.spawn();
    scene.push(&game_object,model);
        
    return game_object;
}

pub fn mono_cube(scene: &mut Query, device: Arc<Device>) -> GameObject {
    let vertices: Vec<Vertex> = vec![
        // left face (white)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // right face (yellow)
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // top face (orange)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // bottom face (red)
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // nose face (blue)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // tail face (green)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
    ];

    let model = Model::new_from_array(Arc::clone(&device),vertices,Vec::new());

    let game_object = scene.spawn();
    scene.push(&game_object,model);
        
    return game_object;
}

pub fn model_cube(scene: &mut Query, device: Arc<Device>) -> Model {
    let vertices: Vec<Vertex> = vec![
        // left face (white)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // right face (yellow)
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // top face (orange)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // bottom face (red)
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // nose face (blue)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // tail face (green)
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
    ];

    let model = Model::new_from_array(Arc::clone(&device),vertices,Vec::new());

    return model;
}*/