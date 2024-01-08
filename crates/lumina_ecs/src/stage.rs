/*use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Mutex},
    thread, rc::Rc,
};

use lumina_object::{
    game_object::{GameObject, Component},
    transform::Transform,
    entity::Entity,
};

pub struct Stage {
    name: String,
    members: Arc<RwLock<HashMap<u32, Rc<Entity>>>>,
}

impl Stage {
    pub fn new(name: String) -> Self {
        Self {
            name,
            members: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create(&self) {
        let game_object = GameObject::create_game_object();
        let entity = Rc::new(Entity::new());

        self.members.write().unwrap().insert(game_object.get_id(), entity);
    }

    pub fn update(&self, fps: f32) {
        let delta_time = 1.0 / fps;

        let num_cpus = num_cpus::get().max(1);

        let handles: Vec<_> = (0..num_cpus)
            .map(|i| {
                let members_read_lock = Arc::clone(&self.members);

                thread::spawn(move || {
                    let len = members_read_lock.read().unwrap().len();
                    let start = i * len / num_cpus;
                    let end = if i == num_cpus - 1 { len } else { (i + 1) * len / num_cpus };

                    // Logic using the chunk of entities
                    for (_, entity) in members_read_lock.read().unwrap().iter().skip(start).take(end - start) {
                        // Your threaded logic here
                        println!("{:?}", entity);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        // Perform any post-processing if needed
    }

}
*/

    /*pub fn update_component(&self,app:&App,game_object:&GameObject){
        let components = 
    }



    pub fn spawn(&mut self) -> GameObject {
        let game_object = GameObject::create_game_object();
        let mut entity = Entity::new();

        entity.add_component(Transform::default());

        self.group..insert(game_object.get_id(), entity);
        return game_object;
    }

    pub fn push<T: Component + Send + 'static>(&mut self, game_object: &GameObject, component: T) {
        self.entities
            .get_mut(&game_object.get_id())
            .unwrap()
            .add_component(component);
    }

    pub fn kill(&mut self, game_object: &GameObject) {
        self.entities.remove_entry(&game_object.get_id());
    }


    pub fn query_entity<'a>(&'a self,game_object: &GameObject) -> Option<&'a Entity>{
        return Some(self.entities.get(&game_object.get_id()).unwrap());
    }

    pub fn query<'a, T: Component + Send + 'static>(&'a self, game_object: &GameObject) -> Option<&'a T> {
        self.entities
            .get(&game_object.get_id())
            .and_then(|entity| entity.get_component::<T>())
    }

    pub fn query_mut<'a, T: Component + Send + 'static>(
        &'a mut self,
        game_object: &GameObject,
    ) -> Option<&'a mut T> {
        self.entities
            .get_mut(&game_object.get_id())
            .and_then(|entity| entity.get_mut_component::<T>())
    }

    pub fn query_all<'a, T: Component + Send + 'static>(&'a self, game_object: &GameObject) -> Vec<&'a T> {
        self.entities
            .get(&game_object.get_id())
            .and_then(|entity| Some(entity.get_components::<T>()))
            .unwrap()
    }

    pub fn query_all_mut<'a, T: Component + Send + 'static>(
        &'a mut self,
        game_object: &GameObject,
    ) -> Vec<&'a mut T> {
        self.entities
            .get_mut(&game_object.get_id())
            .and_then(|entity| Some(entity.get_mut_components::<T>()))
            .unwrap()
    }


    pub fn query_all_components<'a>(&'a mut self,game_object: &GameObject) -> &'a mut HashMap<TypeId, Box<dyn Any + Send>> {
        return self.entities.get_mut(&game_object.get_id()).unwrap().get_all_components();
    }*/
