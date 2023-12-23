use std::{any::{TypeId, Any}, collections::HashMap};

use crate::game_object::Component;

#[derive(Debug)]
pub struct Entity {
    components: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl Entity {
    pub fn add_component<T: Component + 'static + Send>(&mut self, component: T) {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn has_component<T: Component + 'static + Send>(&self) -> bool {
        return self.components.contains_key(&TypeId::of::<T>());
    }

    pub fn get_component<T: Component + 'static + Send>(&self) -> Option<&T> {
        if let Some(component) = self.components.get(&TypeId::of::<T>()) {
            Some(component.downcast_ref::<T>().unwrap())
        } else {
            None
        }
    }

    pub fn get_components<T: Component + 'static + Send>(&self) -> Vec<&T> {
        return self
            .components
            .values()
            .filter_map(|component| component.downcast_ref::<T>())
            .collect();
    }

    pub fn get_mut_component<T: Component + 'static + Send>(&mut self) -> Option<&mut T> {
        if let Some(component) = self.components.get_mut(&TypeId::of::<T>()) {
            Some(component.downcast_mut::<T>().unwrap())
    } else {
            None
        }
    }

    pub fn get_mut_components<T: Component + 'static + Send>(&mut self) -> Vec<&mut T> {
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