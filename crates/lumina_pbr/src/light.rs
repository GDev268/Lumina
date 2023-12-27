pub enum LightType {
    DIRECTIONAL,
    POINT,
    SPOT,
}

pub enum ShadowType {
    NO_SHADOW = 0,
    LIGHT_SHADOW = 1,
}

pub trait Light {
    fn get_light_type(&self) -> &LightType;
    fn change_color(&mut self, new_color: glam::Vec3);
    fn change_intensity(&mut self, new_intensity: f32);
    fn change_shadow_strength(&mut self, new_shadow_strength: f32);
    fn remove_culling_mask_layer(&mut self, mask_name: String);
    fn add_culling_mask_layer(&mut self, mask_name: String);
    fn change_shadow_type(&mut self, new_shadow_type: ShadowType);
    fn change_range(&mut self, new_range: f32) {
        println!("This light doesn't support light range!");
    }
    fn change_spot_size(&mut self, new_spot_size: f32) {
        println!("This light doesn't support spot size!");
    }
}

pub struct DirectionalLight {
    light_type:LightType,
    color: [f32; 3],
    intensity: f32,
    shadow_strength: f32,
    shadow_type: ShadowType,
    culling_mask: Vec<String>,
}

impl DirectionalLight {
    pub fn new() -> Self {
        Self {
            color: [0.0; 3],
            intensity: 0.0,
            shadow_strength: 0.0,
            shadow_type: ShadowType::LIGHT_SHADOW,
            culling_mask: vec![],
            light_type: LightType::DIRECTIONAL,
        }
    }

    pub fn create_raw_light(&self) -> RawDirectionalLight {
        RawDirectionalLight {
            color: self.color,
            intensity: self.intensity
        }
    }
}

impl Light for DirectionalLight {
    fn change_color(&mut self, new_color: glam::Vec3) {
        self.color = new_color.to_array();
    }

    fn change_intensity(&mut self, new_intensity: f32) {
        self.intensity = new_intensity;
    }

    fn change_shadow_strength(&mut self, new_shadow_strength: f32) {
        self.shadow_strength = new_shadow_strength;
    }

    fn remove_culling_mask_layer(&mut self, mask_name: String) {
        let position = self.culling_mask.iter().position(|s| *s == mask_name);

        if position.is_some() {
            self.culling_mask.remove(position.unwrap());
        }
    }

    fn add_culling_mask_layer(&mut self, mask_name: String) {
        self.culling_mask.push(mask_name);
    }

    fn change_shadow_type(&mut self, new_shadow_type: ShadowType) {
        self.shadow_type = new_shadow_type;
    }

    fn get_light_type(&self) -> &LightType {
        return &self.light_type;
    }
}

//ADD THE REST LATER
pub struct RawDirectionalLight {
    color: [f32; 3],
    intensity: f32,
}

impl Default for RawDirectionalLight {
    fn default() -> Self {
        Self {
            color: [0.0; 3],
            intensity: 0.0,
        }
    }
}

pub struct PointLight {
    light_type:LightType,
    color: [f32; 3],
    intensity: f32,
    range: f32,
    shadow_strength: f32,
    shadow_type: ShadowType,
    culling_mask: Vec<String>,
}

impl PointLight {
    pub fn new() -> Self {
        Self {
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            shadow_strength: 0.0,
            shadow_type: ShadowType::LIGHT_SHADOW,
            culling_mask: vec![],
            light_type: LightType::POINT,
        }
    }

    pub fn create_raw_light(&self) -> RawPointLight {
        RawPointLight { color: self.color,intensity: self.intensity, range: self.range,..Default::default() }
    }
}

impl Light for PointLight {
    fn change_color(&mut self, new_color: glam::Vec3) {
        self.color = new_color.to_array();
    }

    fn change_intensity(&mut self, new_intensity: f32) {
        self.intensity = new_intensity;
    }

    fn change_shadow_strength(&mut self, new_shadow_strength: f32) {
        self.shadow_strength = new_shadow_strength;
    }

    fn remove_culling_mask_layer(&mut self, mask_name: String) {
        let position = self.culling_mask.iter().position(|s| *s == mask_name);

        if position.is_some() {
            self.culling_mask.remove(position.unwrap());
        }
    }

    fn add_culling_mask_layer(&mut self, mask_name: String) {
        self.culling_mask.push(mask_name);
    }

    fn change_shadow_type(&mut self, new_shadow_type: ShadowType) {
        self.shadow_type = new_shadow_type;
    }

    fn change_range(&mut self, new_range: f32) {
        self.range = new_range;
    }

    fn get_light_type(&self) -> &LightType {
        return &self.light_type;
    }
}

pub struct RawPointLight {
    color: [f32; 3],
    _padding1: f32,
    intensity: f32,
    _padding2: f32,
    range: f32,
}

impl Default for RawPointLight {
    fn default() -> Self {
        Self {
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            _padding1: 0.0,
            _padding2: 0.0,
        }
    }
}

pub struct SpotLight {
    light_type:LightType,
    color: [f32; 3],
    intensity: f32,
    range: f32,
    spot_size: f32,
    shadow_strength: f32,
    shadow_type: ShadowType,
    culling_mask: Vec<String>,
}

impl SpotLight {
    pub fn new() -> Self {
        Self {
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            spot_size: 0.0,
            shadow_strength: 0.0,
            shadow_type: ShadowType::LIGHT_SHADOW,
            culling_mask: vec![],
            light_type: LightType::SPOT,
        }
    }
}

impl Light for SpotLight {
    fn change_color(&mut self, new_color: glam::Vec3) {
        self.color = new_color.to_array();
    }

    fn change_intensity(&mut self, new_intensity: f32) {
        self.intensity = new_intensity;
    }

    fn change_shadow_strength(&mut self, new_shadow_strength: f32) {
        self.shadow_strength = new_shadow_strength;
    }

    fn remove_culling_mask_layer(&mut self, mask_name: String) {
        let position = self.culling_mask.iter().position(|s| *s == mask_name);

        if position.is_some() {
            self.culling_mask.remove(position.unwrap());
        }
    }

    fn add_culling_mask_layer(&mut self, mask_name: String) {
        self.culling_mask.push(mask_name);
    }

    fn change_shadow_type(&mut self, new_shadow_type: ShadowType) {
        self.shadow_type = new_shadow_type;
    }

    fn change_range(&mut self, new_range: f32) {
        self.range = new_range;
    }

    fn change_spot_size(&mut self, new_spot_size: f32) {
        self.spot_size = new_spot_size;
    }

    fn get_light_type(&self) -> &LightType {
        return &self.light_type;
    }
}

struct RawSpotLight {
    color: [f32; 3],
    _padding1: f32,
    intensity: f32,
    _padding2: f32,
    range: f32,
    _padding3: f32,
    spot_size: f32,
}

impl Default for RawSpotLight {
    fn default() -> Self {
        Self {
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            spot_size: 0.0,
            _padding1: 0.0,
            _padding2: 0.0,
            _padding3: 0.0,
        }
    }
}
