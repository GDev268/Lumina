use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use lumina_core::device::Device;

static mut CURRENT_ID: u32 = 0;

pub trait Component: Any {}

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
            CURRENT_ID += 1;
        }

        return game_object;
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }
}

