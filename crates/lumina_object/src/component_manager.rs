use std::{
    any::TypeId,
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
    thread,
};

use crate::{game_object::{Component, GameObject}, transform::Transform};
use num_cpus::*;

pub struct ComponentManager {
    pub components: Arc<RwLock<HashMap<u32, HashMap<TypeId, Box<dyn Component>>>>>,
    query_mut_components: HashMap<u32, Vec<Box<dyn Component>>>,
    query_components: Vec<Box<dyn Component>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::new())),
            query_mut_components: HashMap::new(),
            query_components: Vec::new(),
        }
    }

    pub fn spawn(&mut self) -> GameObject {
        let game_object = GameObject::create_game_object();

        let mut components: HashMap<TypeId, Box<dyn Component>> = HashMap::new();

        let component = Transform::default();
        components.insert(TypeId::of::<Transform>(), Box::new(component));

        self.components
            .write()
            .unwrap()
            .insert(game_object.get_id(), components);

        game_object
    }

    pub fn kill(&mut self,game_object:&GameObject) {
        self.components.write().unwrap().remove_entry(&game_object.get_id());
    }

    pub fn push<T: Component + Send + 'static>(&mut self, game_object: &GameObject, component: T) {
        self.components.write().unwrap()
            .get_mut(&game_object.get_id())
            .unwrap()
            .insert(TypeId::of::<T>(), Box::new(component));
    }


    pub fn query<'a, T: Component>(&'a mut self, id: u32) -> &'a T {
        let index = self.query_components.len();
        let read_lock = self.components.read().unwrap();
        if let Some(component) = read_lock.get(&id).unwrap().get(&TypeId::of::<T>()) {
            self.query_components.push(component.deref().clone());
            drop(read_lock);
        }

        self.query_components
            .get(index)
            .unwrap()
            .as_any()
            .downcast_ref::<T>()
            .unwrap()
    }

    pub fn query_mut<'a, T: Component>(&'a mut self, id: u32) -> &'a mut T {
        let index = self.query_mut_components.get(&id).unwrap().len();
        let mut read_lock = self.components.write().unwrap();
        if let Some(component) = read_lock.get_mut(&id).unwrap().get_mut(&TypeId::of::<T>()) {
            self.query_mut_components
                .get_mut(&id)
                .unwrap()
                .push(component.deref_mut().clone());
            drop(read_lock);
        }

        self.query_mut_components
            .get_mut(&id)
            .unwrap()
            .get_mut(index)
            .unwrap()
            .as_mut_any()
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn publish_changes(&mut self) {
        for (id, components) in self.query_mut_components.iter_mut() {
            for component in components.iter_mut() {
                let mut write_lock = self.components.write().unwrap();

                for components in write_lock.get_mut(id).iter_mut() {
                    for (_, mut root_component) in components.iter_mut() {
                        if root_component.get_id() == component.get_id() {
                            root_component = component;
                        }
                    }
                }
            }
        }

        self.query_mut_components.clear();
        self.query_components.clear();
    }

    pub fn update(&mut self) {
        let num_cpus = num_cpus::get().max(1);

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let members_read_lock = Arc::clone(&self.components);

                thread::spawn(move || {
                    let len = members_read_lock.read().unwrap().len();
                    let start = i * len / num_cpus;
                    let end = if i == num_cpus - 1 {
                        len
                    } else {
                        (i + 1) * len / num_cpus
                    };

                    for (_, components) in members_read_lock
                        .write()
                        .unwrap()
                        .iter_mut()
                        .skip(start)
                        .take(end - start)
                    {
                        for (_, component) in components.iter_mut() {
                            println!("{:?}", component.get_id());
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
