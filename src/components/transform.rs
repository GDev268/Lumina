use super::game_object::Component;

pub struct Transform {
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Vec3,
}

impl Transform {
    pub fn get_mat4(&self) -> glam::Mat4 {
        let c3: f32 = self.rotation.x.cos();
        let s3: f32 = self.rotation.x.sin();
        let c2: f32 = self.rotation.y.cos();
        let s2: f32 = self.rotation.y.sin();
        let c1: f32 = self.rotation.z.cos();
        let s1: f32 = self.rotation.z.sin();

        return glam::mat4(
            glam::vec4(
                self.scale.x * (c1 * c3 + s1 * s2 * s3),
                self.scale.x * (c2 * s3),
                self.scale.x * (c1 * s2 * s3 - c3 * s1),
                0.0,
            ),
            glam::vec4(
                self.scale.y * (c3 * s1 * s2 - c1 * s3),
                self.scale.y * (c2 * c3),
                self.scale.y * (c1 * c3 * s2 + s1 * s3),
                0.0,
            ),
            glam::vec4(
                self.scale.z * (c2 * s1),
                self.scale.z * (-s2),
                self.scale.z * (c1 * c2),
                0.0,
            ),
            glam::vec4(
                self.translation.x,
                self.translation.y,
                self.translation.z,
                1.0,
            ),
        );
    }

    pub fn get_normal_matrix(&self) -> glam::Mat4 {
        let c3: f32 = self.rotation.z.cos();
        let s3: f32 = self.rotation.z.sin();
        let c2: f32 = self.rotation.x.cos();
        let s2: f32 = self.rotation.x.sin();
        let c1: f32 = self.rotation.y.cos();
        let s1: f32 = self.rotation.y.sin();
        let inverse_scale: glam::Vec3 = 1.0 / self.scale;

        return glam::mat4(
            glam::vec4(
                inverse_scale.x * (c1 * c3 + s1 * s2 * s3),
                inverse_scale.x * (c2 * s3),
                inverse_scale.x * (c1 * s2 * s3 - c3 * s1),
                1.0,
            ),
            glam::vec4(
                inverse_scale.y * (c3 * s1 * s2 - c1 * s3),
                inverse_scale.y * (c2 * c3),
                inverse_scale.y * (c1 * c3 * s2 + s1 * s3),
                1.0,
            ),
            glam::vec4(
                inverse_scale.z * (c2 * s1),
                inverse_scale.z * (-s2),
                inverse_scale.z * (c1 * c2),
                1.0,
            ),
            glam::vec4(1.0, 1.0, 1.0, 1.0),
        );
    }

    pub fn default() -> Self {
        return Self {
            translation: glam::Vec3::default(),
            scale: glam::Vec3::default(),
            rotation: glam::Vec3::default(),
        };
    }
}

impl Component for Transform {}