use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{delete_component_id, entity::Entity, game_object::{Component, GameObject}, transform::Transform};

pub struct ComponentManager {
    entities: Arc<Mutex<HashMap<u32, Arc<Mutex<Entity>>>>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn spawn(&self) -> GameObject {
        let game_object = GameObject::create_game_object();
        let mut entity = Entity::new();

        entity.add_component(Transform::default());

        let entity = Arc::new(Mutex::new(entity));
        let mut entities = self.entities.lock().unwrap();
        entities.insert(game_object.get_id(), entity);

        game_object
    }

    pub fn push<T: Component + 'static>(&self, game_object: &GameObject, component: T) {
        if let Some(entity) = self.entities.lock().unwrap().get(game_object.get_id()) {
            entity.lock().unwrap().add_component(component);
        }
    }

    pub fn kill(&self, game_object: &GameObject) {
        delete_component_id(game_object.get_id());
        self.entities.lock().unwrap().remove(&game_object.get_id());
    }

    pub fn query_entity(&self, game_object: &GameObject) -> Option<Arc<Mutex<Entity>>> {
        self.entities.lock().unwrap().get(&game_object.get_id()).cloned()
    }

    pub fn query<T: Component + 'static>(&self, game_object: &GameObject) -> Option<Arc<Mutex<T>>> {
        if let Some(entity) = self.entities.lock().unwrap().get(&game_object.get_id()) {
            if let Some(component) = entity.lock().unwrap().get_component::<T>() {
                return Some(Arc::clone(component));
            }
        }
        None
    }

    pub fn query_mut<T: Component + 'static>(&self, game_object: &GameObject) -> Option<Arc<Mutex<T>>> {
        if let Some(entity) = self.entities.lock().unwrap().get(&game_object.get_id()) {
            if let Some(component) = entity.lock().unwrap().get_mut_component::<T>() {
                return Some(Arc::clone(component));
            }
        }
        None
    }

    pub fn query_all<T: Component + 'static>(&self, game_object: &GameObject) -> Vec<Arc<Mutex<T>>> {
        if let Some(entity) = self.entities.lock().unwrap().get(&game_object.get_id()) {
            return entity.lock().unwrap().get_components::<T>().iter().map(|c| Arc::clone(c)).collect();
        }
        Vec::new()
    }

    pub fn query_all_mut<T: Component + 'static>(&self, game_object: &GameObject) -> Vec<Arc<Mutex<T>>> {
        if let Some(entity) = self.entities.lock().unwrap().get(&game_object.get_id()) {
            return entity.lock().unwrap().get_mut_components::<T>().iter().map(|c| Arc::clone(c)).collect();
        }
        Vec::new()
    }
}
