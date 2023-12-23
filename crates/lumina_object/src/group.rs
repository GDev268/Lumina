use std::{collections::HashMap, sync::{Mutex, Arc}, thread, borrow::{BorrowMut, Borrow}};
use num_cpus;

use crate::{entity::Entity, transform::Transform};

struct SystemGroup {
    entities: Arc<Mutex<HashMap<u32, Entity>>>,
}

impl SystemGroup {
    pub fn new() -> Self {
        Self { entities: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn add_member(&mut self, id: u32, entity: Entity) {
        self.entities.borrow_mut().lock().unwrap().insert(id, entity);
    }

    pub fn remove_member(&mut self, id: u32) {
        self.entities.borrow_mut().lock().unwrap().remove(&id);
    }

    pub fn update(&mut self) {
        let num_cpus = num_cpus::get().max(1);

        let chunk_size = self.entities.lock().as_ref().borrow().unwrap().len() / num_cpus;

        let handles: Vec<_> = (0..num_cpus).map(|i| {
            let entity_clone = Arc::clone(&self.entities);
            let start = i * chunk_size;
            let end = if i == num_cpus - 1 {
                self.entities.lock().as_ref().borrow().unwrap().len()
            } else {
                (i + 1) * chunk_size
            };
            let mut cloned_entities = Arc::clone(&self.entities);

            thread::spawn(move || {
                for entity in cloned_entities
                    .borrow_mut()
                    .lock()
                    .unwrap()
                    .values()
                    .skip(start)
                    .take(end - start)
                {

                }
            })
        })
        .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn render(&mut self) {
        // Your rendering logic here
    }
}

unsafe impl Send for SystemGroup {}

/*
use num_cpus;
use rand::Rng;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

const VALUES: usize = 1000;

fn main() {
    // Create entities with sequential IDs
    let entities: HashMap<usize, Entity> = (0..VALUES)
        .map(|id| (rand::thread_rng().gen::<usize>(), Entity::new(id)))
        .collect();

    // Convert the HashMap to a vector
    let entities_vec: Vec<_> = entities.values().cloned().collect();

    let num_threads = num_cpus::get().max(1);
    let dynamic_chunk_size = VALUES / num_threads;

    let instant = Instant::now();
    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let entities_chunk: Vec<_> = entities_vec[i * dynamic_chunk_size..(i + 1) * dynamic_chunk_size].to_vec();
            thread::spawn(move || {
                for entity in entities_chunk {
                    println!("Entity ID: {}", entity.id);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Calculation Duration: {:?}ms", instant.elapsed().as_millis());
}


 */