use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use lumina_object::{
    entity::Entity,
    game_object::{Component, GameObject},
    transform::Transform,
};

pub struct Query {
    pub entities: Arc<RwLock<HashMap<u32, Arc<RwLock<Entity>>>>>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn spawn(&self) -> GameObject {
        let game_object = GameObject::create_game_object();
        let entity = Arc::new(RwLock::new(Entity::new()));
        entity.write().unwrap().add_component(Transform::default());

        self.entities.write().unwrap().insert(game_object.get_id(), entity.clone());

        game_object
    }

    pub fn spawn_with_id(&self,id:u32) -> GameObject {
        let game_object = GameObject::create_game_object_with_id(id);
        let entity = Arc::new(RwLock::new(Entity::new()));

        self.entities.write().unwrap().insert(game_object.get_id(), entity.clone());

        game_object
    }


    pub fn push<T: Component + 'static>(&self, game_object: &GameObject, component: T) {
        if let Some(entity) = self.entities.read().unwrap().get(&game_object.get_id()) {
            entity.write().unwrap().add_component(component);
        }
    }

    pub fn kill(&self, game_object: &GameObject) {
        self.entities.write().unwrap().remove(&game_object.get_id());
    }

    pub fn query_entity(&self, game_object: &GameObject) -> Option<Arc<RwLock<Entity>>> {
        self.entities.read().unwrap().get(&game_object.get_id()).cloned()
    }

}
