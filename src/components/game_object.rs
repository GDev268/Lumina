use ash::vk;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::engine::device::Device;

static mut CURRENT_ID: u32 = 0;

pub type Component = Box<dyn Any>;

pub struct Transform {
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Vec3,
}

impl Transform {
    pub fn get_mat4(&self) -> glam::Mat4 {
        let c3: f32 = self.rotation.z.cos();
        let s3: f32 = self.rotation.z.sin();
        let c2: f32 = self.rotation.x.cos();
        let s2: f32 = self.rotation.x.sin();
        let c1: f32 = self.rotation.y.cos();
        let s1: f32 = self.rotation.y.sin();

        return glam::mat4(
            glam::vec4(
                self.scale.x * (c1 * c3 + s1 * s2 * s3),
                self.scale.x * (c2 * s3),
                self.scale.x * (c1 * s2 * s3 - c3 * s1),
                0.0,
            ),
            glam::vec4(
                self.scale.y * (c3 * s1 * s2 - c1 * s3),
                self.scale.y * (c2 * c3),
                self.scale.y * (c1 * c3 * s2 + s1 * s3),
                0.0,
            ),
            glam::vec4(
                self.scale.z * (c2 * s1),
                self.scale.z * (-s2),
                self.scale.z * (c1 * c2),
                0.0,
            ),
            glam::vec4(
                self.translation.x,
                self.translation.y,
                self.translation.z,
                1.0,
            ),
        );
    }

    pub fn get_normal_matrix(&self) -> glam::Mat4 {
        let c3: f32 = self.rotation.z.cos();
        let s3: f32 = self.rotation.z.sin();
        let c2: f32 = self.rotation.x.cos();
        let s2: f32 = self.rotation.x.sin();
        let c1: f32 = self.rotation.y.cos();
        let s1: f32 = self.rotation.y.sin();
        let inverse_scale: glam::Vec3 = 1.0 / self.scale;

        return glam::mat4(
            glam::vec4(
                inverse_scale.x * (c1 * c3 + s1 * s2 * s3),
                inverse_scale.x * (c2 * s3),
                inverse_scale.x * (c1 * s2 * s3 - c3 * s1),
                1.0,
            ),
            glam::vec4(
                inverse_scale.y * (c3 * s1 * s2 - c1 * s3),
                inverse_scale.y * (c2 * c3),
                inverse_scale.y * (c1 * c3 * s2 + s1 * s3),
                1.0,
            ),
            glam::vec4(
                inverse_scale.z * (c2 * s1),
                inverse_scale.z * (-s2),
                inverse_scale.z * (c1 * c2),
                1.0,
            ),
            glam::vec4(1.0, 1.0, 1.0, 1.0),
        );
    }

    pub fn default() -> Self {
        return Self {
            translation: glam::Vec3::default(),
            scale: glam::Vec3::default(),
            rotation: glam::Vec3::default(),
        };
    }
}

pub struct GameObject {
    id: u32,
    tag: String,
    layer: String,
    name: String,
}

impl GameObject {
    pub fn new(id: u32) -> Self {
        let layer = String::from("Default");
        let tag = String::from("Entity");
        let name = String::default();

        return Self {
            id,
            layer,
            tag,
            name,
        };
    }

    pub fn create_game_object() -> Self {
        let game_object: GameObject = unsafe { GameObject::new(CURRENT_ID) };

        unsafe {
            CURRENT_ID = CURRENT_ID + 1;
        }

        return game_object;
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }
}


pub struct Entity {
    components: HashMap<TypeId, Component>,
}

impl Entity {
    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn has_component<T: 'static>(&self) -> bool {
        return self.components.contains_key(&TypeId::of::<T>());
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        if let Some(component) = self.components.get(&TypeId::of::<T>()) {
            Some(component.downcast_ref::<T>().unwrap())
        } else {
            None
        }
    }

    pub fn get_components<T: 'static>(&self) -> Vec<&T> {
        return self
            .components
            .values()
            .filter_map(|component| component.downcast_ref::<T>())
            .collect();
    }

    pub fn get_mut_component<T: 'static>(&mut self) -> Option<&mut T> {
        if let Some(component) = self.components.get_mut(&TypeId::of::<T>()) {
            Some(component.downcast_mut::<T>().unwrap())
        } else {
            None
        }
    }

    pub fn get_mut_components<T: 'static>(&mut self) -> Vec<&mut T> {
        return self
            .components
            .values_mut()
            .filter_map(|component| component.downcast_mut::<T>())
            .collect();
    }

    pub fn new() -> Self{
        return Self { components: HashMap::new() };
    }

}
