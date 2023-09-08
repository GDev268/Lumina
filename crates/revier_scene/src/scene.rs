use std::{any::Any, collections::HashMap};

use revier_object::{
    game_object::{Component, GameObject},
    transform::Transform,
    entity::Entity
};

pub struct Scene {
    pub entities: HashMap<u32, Entity>,
}

impl Scene {
    pub fn new() -> Self {
        return Self {
            entities: HashMap::new(),
        };
    }

    pub fn spawn(&mut self) -> GameObject {
        let game_object = GameObject::create_game_object();
        let mut entity = Entity::new();

        entity.add_component(Transform::default());

        self.entities.insert(game_object.get_id(), entity);
        return game_object;
    }

    pub fn push<T: Component + 'static>(&mut self, game_object: &GameObject, component: T) {
        self.entities
            .get_mut(&game_object.get_id())
            .unwrap()
            .add_component(component);
    }

    pub fn kill(&mut self, game_object: &GameObject) {
        self.entities.remove_entry(&game_object.get_id());
    }

    pub fn query<'a, T: Component + 'static>(&'a self, game_object: &GameObject) -> Option<&'a T> {
        self.entities
            .get(&game_object.get_id())
            .and_then(|entity| entity.get_component::<T>())
    }

    pub fn query_mut<'a, T: Component + 'static>(
        &'a mut self,
        game_object: &GameObject,
    ) -> Option<&'a mut T> {
        self.entities
            .get_mut(&game_object.get_id())
            .and_then(|entity| entity.get_mut_component::<T>())
    }

    pub fn query_all<'a, T: Component + 'static>(&'a self, game_object: &GameObject) -> Vec<&'a T> {
        self.entities
            .get(&game_object.get_id())
            .and_then(|entity| Some(entity.get_components::<T>()))
            .unwrap()
    }

    pub fn query_all_mut<'a, T: Component + 'static>(
        &'a mut self,
        game_object: &GameObject,
    ) -> Vec<&'a mut T> {
        self.entities
            .get_mut(&game_object.get_id())
            .and_then(|entity| Some(entity.get_mut_components::<T>()))
            .unwrap()
    }
}
