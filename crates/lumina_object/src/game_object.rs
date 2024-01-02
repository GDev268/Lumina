use ash::vk;
use lumina_bundle::ResourcesBundle;
use rand::Rng;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use lumina_core::device::Device;
use lazy_static::lazy_static;

static mut EXISTING_IDS: Vec<u32> = vec![];

pub trait Component: Any {
    fn max_component_count() -> Option<usize> {
        return None;
    }
}

#[derive(Debug)]
pub struct GameObject{
    id: u32,
    tag: String,
    layer: String,
    name: String,
    parent:Option<u32>,
    children:Vec<u32>
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
            parent: None,
            name,
            children:Vec::new()
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

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn push_to_gameobject(&mut self,game_object:&mut GameObject) {
        self.parent = Some(game_object.get_id());
        game_object.children.push(self.get_id());
    }
}

