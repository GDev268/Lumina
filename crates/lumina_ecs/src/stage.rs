use std::{
    borrow::{Borrow, BorrowMut},
    sync::Arc,
    thread,
    time::Instant,
};

use lumina_object::group::SystemGroup;

pub struct Stage {
    name: String,
    group: SystemGroup,
}

impl Stage {
    pub fn new(name: String) -> Self {
        Self {
            name,
            group: SystemGroup::new(),
        }
    }

    pub fn create(&mut self) {}

    pub fn update(&mut self, fps: f32) {
        let delta_time = 1.0 / fps;

        let num_cpus = num_cpus::get().max(1);

        let chunk_size = self.group.members.lock().as_ref().borrow().unwrap().len() / num_cpus;

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let start = i * chunk_size;
                let end = if i == num_cpus - 1 {
                    self.group.members.lock().as_ref().borrow().unwrap().len()
                } else {
                    (i + 1) * chunk_size
                };
                let mut cloned_entities = Arc::clone(&self.group.members);

                thread::spawn(move || {
                    for entity in cloned_entities
                        .borrow_mut()
                        .lock()
                        .unwrap()
                        .values()
                        .skip(start)
                        .take(end - start)
                    {}
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
