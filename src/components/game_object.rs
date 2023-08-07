static mut CURRENT_ID: u32 = 0;

pub struct TransformComponent {
    translation: glam::Vec3,
    scale: glam::Vec3,
    rotation: glam::Vec3,
}

impl TransformComponent {
    pub fn get_mat4() -> glam::Mat4 {
        return glam::Mat4::default();
    }

    pub fn get_normal_matrix() -> glam::Mat3 {
        return glam::Mat3::default();
    }

    pub fn default() -> Self {
        return Self {
            translation: glam::Vec3::default(),
            scale: glam::Vec3::default(),
            rotation: glam::Vec3::default(),
        };
    }
}

pub struct GameObject {
    pub id: u32,
    pub tag: String,
    pub layer: String,
    pub transform: TransformComponent,
    pub name:String
}

impl GameObject {
    pub fn new(id: u32) -> Self {
        let layer = String::from("Default");
        let tag = String::from("Entity");
        let transform = TransformComponent::default();
        let name = String::default();

        return Self {
            id,
            layer,
            tag,
            transform,
            name,
        };
    }

    pub fn create_game_object() -> Self {
        let game_object: GameObject = unsafe { GameObject::new(CURRENT_ID) };

        unsafe {
            CURRENT_ID = CURRENT_ID + 1;
        }

        
        return game_object;
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }
}
