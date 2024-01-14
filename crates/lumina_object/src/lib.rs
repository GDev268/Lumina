use rand::Rng;

pub mod entity;
pub mod game_object;
pub mod transform;
pub mod component_manager;

static mut COMPONENT_IDS: Vec<u32> = vec![];

pub fn create_component_id() -> u32 {
    let mut rng = rand::thread_rng();

    let mut random_id:u32 = rng.gen_range(0x0000_0001,0xFFFF_FFFF);

    while unsafe { COMPONENT_IDS.contains(&random_id) }{
        random_id = rng.gen_range(0x0000_0001,0xFFFF_FFFF);
    }

    unsafe{
        COMPONENT_IDS.push(random_id);
    }

    random_id
}

pub fn delete_component_id(id:u32) {
    unsafe{
        COMPONENT_IDS.remove(COMPONENT_IDS.iter().position(|e| *e == id).unwrap());
    }
}