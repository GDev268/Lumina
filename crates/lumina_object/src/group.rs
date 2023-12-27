use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use crate::{entity::Entity, transform::Transform};

pub struct SystemGroup {
    pub members: Arc<Mutex<HashMap<u32, Entity>>>,
}

impl SystemGroup {
    pub fn new() -> Self {
        Self {
            members: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_member(&mut self, id: u32, entity: Entity) {
        self.members
            .borrow_mut()
            .lock()
            .unwrap()
            .insert(id, entity);
    }

    pub fn remove_member(&mut self, id: u32) {
        self.members.borrow_mut().lock().unwrap().remove(&id);
    }

}

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
