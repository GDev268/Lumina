use std::rc::Rc;

use lumina_core::{device::Device, Vertex3D};
use lumina_object::{game_object::GameObject, component_manager::ComponentManager};
use lumina_render::{mesh::Mesh, model::Model};


pub fn cube(scene: &mut ComponentManager, device: Rc<Device>) -> GameObject {
    let vertices: Vec<Vertex3D> = vec![
        // left face (white)
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // right face (yellow)
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // top face (orange)
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // bottom face (red)
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // nose face (blue)
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // tail face (green)
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
    ];

    let model = Model::new_from_array(Rc::clone(&device),vertices,Vec::new());

    let game_object = scene.spawn();
    scene.push(&game_object,model);
        
    return game_object;
}

pub fn mono_cube(scene: &mut ComponentManager, device: Rc<Device>) -> GameObject {
    let vertices: Vec<Vertex3D> = vec![
        // left face (white)
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(-1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // right face (yellow)
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(1.0, 0.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // top face (orange)
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, -1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // bottom face (red)
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 1.0, 0.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // nose face (blue)
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, 0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, 1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        // tail face (green)
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 1.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(-0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(0.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, -0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 0.0).into(),
        },
        Vertex3D {
            position: glam::Vec3::new(0.5, 0.5, -0.5).into(),
            normal: glam::Vec3::new(0.0, 0.0, -1.0).into(),
            uv: glam::Vec2::new(1.0, 1.0).into(),
        },
    ];

    let model = Model::new_from_array(Rc::clone(&device),vertices,Vec::new());

    let game_object = scene.spawn();
    scene.push(&game_object,model);
        
    return game_object;
}