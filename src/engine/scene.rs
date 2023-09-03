use std::{any::Any, collections::HashMap};

use num::complex::ComplexFloat;

use crate::components::game_object::{self, GameObject,Component, Transform, Entity};

struct Scene{
    pub entities:HashMap<u32,Entity>
}

impl Scene{
    pub fn new() -> Self {
        return Self { entities: HashMap::new() };
    }

    pub fn spawn(&mut self,components:Vec<Box<dyn Any + 'static>>) -> GameObject {
        let game_object = GameObject::create_game_object();
        let mut entity = Entity::new();
        let mut has_transform = false;


        for component in components {
            if component.is::<Transform>() {
                has_transform = true;
            }
            entity.add_component(component);
        }

        if !has_transform {
            entity.add_component(Transform::default());
        }


        self.entities.insert(game_object.get_id(), entity);
        return game_object;
    }

    pub fn kill(&mut self,game_object:&GameObject){
        self.entities.remove_entry(&game_object.get_id());
    }
    
    pub fn query<'a, T: 'static>(&'a mut self, game_object: &GameObject, mutable: bool) -> Option<&'a T> {
        let result = self.entities.get_mut(&game_object.get_id()).and_then(|entity| {
            if mutable {
                entity.get_mut_component::<T>().map(|c| c as &T)
            } else {
                entity.get_component::<T>()
            }
        });
    
        result
    }
    
    pub fn query_multiple<'a, T: 'static>(
        &'a mut self,
        game_object: &GameObject,
        mutable: bool,
    ) -> Option<Vec<&'a T>> {
        let result = self.entities.get_mut(&game_object.get_id()).map(|entity| {
            if mutable {
                let mut components = Vec::new();
                for c in entity.get_mut_components::<T>() {
                    components.push(c as &T);
                }
                components
            } else {
                entity.get_components::<T>()
            }
        });
    
        result
    }
  
    
    


    
}