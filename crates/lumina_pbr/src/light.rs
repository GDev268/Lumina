use lumina_core::RawLight;
use lumina_object::{game_object::Component, transform::Transform};

#[repr(u32)]
#[derive(Debug,Clone, Copy)]
pub enum LightType {
    DIRECTIONAL = 0,
    POINT = 1,
    SPOT = 2,
}

pub struct Light {
    light_type:LightType,
    color: [f32; 3],
    intensity: f32,
    range: f32,
    spot_size: f32,
}

impl Light {
    pub fn new() -> Self {
        Self {
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            spot_size: 0.0,
            light_type: LightType::SPOT,
        }
    }

    pub fn create_raw_light(&self,id:&u32,transform:&Transform) -> RawLight {
        let linear = 1.2833333333333333333333333333333 + ((-0.05833333333333333333333333333333) * self.range);
        let quadratic = 2.0888888888888888888888888888888 + ((-0.04074074074074074074074074074074) * self.range);

        RawLight {
            color: self.color,
            position: transform.translation.to_array(),
            rotation: transform.rotation.to_array(),
            linear,
            quadratic,
            intensity: self.intensity,
            spot_size: self.spot_size,
            light_type: self.light_type as u32,
            _padding1: 0,
            _padding2: 0,
            _padding3: 0
        }
    }

    pub fn change_color(&mut self, new_color: glam::Vec3) {
        self.color = new_color.to_array();
    }

    pub fn change_intensity(&mut self, new_intensity: f32) {
        self.intensity = new_intensity;
    }

    pub fn change_range(&mut self, new_range: f32) {
        self.range = new_range;
    }

    pub fn change_spot_size(&mut self, new_spot_size: f32) {
        self.spot_size = new_spot_size;
    }

    pub fn change_light_type(&mut self,light_type:u32) {
        self.light_type = unsafe { std::mem::transmute(light_type) };
    }
}

impl Component for Light {
    fn convert_to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "light_type": self.light_type as u32,
            "color": self.color.to_vec(),
            "intensity": self.intensity,
            "range": self.range,
            "spot_size": self.spot_size
        })
    }
}