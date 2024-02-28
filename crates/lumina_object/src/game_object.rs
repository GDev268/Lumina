use ash::vk;
use rand::Rng;
use serde_json::Value;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use lumina_core::device::Device;
use lazy_static::lazy_static;

static mut EXISTING_IDS: Vec<u32> = vec![];

pub trait Component: Any + Send + Sync {
    fn convert_to_json(&self,id:u32) -> Value {
        serde_json::json!({})
    }
}

#[derive(Debug,Clone)]
pub struct GameObject {
    id: u32,
    tag: String,
    layer: String,
    name: String,
}

impl GameObject {
    fn new(id: u32) -> Self {
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
        let mut rng = rand::thread_rng();

        let mut random_id:u32 = rng.gen_range(0x0000_0001,0xFFFF_FFFF);

        while unsafe { EXISTING_IDS.contains(&random_id) }{
            random_id = rng.gen_range(0x0000_0001,0xFFFF_FFFF);
        }

        let game_object: GameObject = unsafe { GameObject::new(random_id) };

        unsafe {
            EXISTING_IDS.push(random_id);
        }

        return game_object;
    }

    pub fn create_game_object_with_id(id:u32) -> Self {
        let game_object: GameObject = unsafe { GameObject::new(id) };

        unsafe {
            EXISTING_IDS.push(id);
        }

        return game_object;
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }
}

