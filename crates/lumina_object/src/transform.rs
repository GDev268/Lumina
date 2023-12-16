use super::game_object::Component;

pub struct Transform {
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Vec3, // Rotation angles for X, Y, and Z axes
}

impl Transform {
    pub fn get_mat4(&self) -> glam::Mat4 {
        let c1_x = self.rotation.x.to_radians().cos();
        let s1_x = self.rotation.x.to_radians().sin();
        let c2_y = self.rotation.y.to_radians().cos();
        let s2_y = self.rotation.y.to_radians().sin();
        let c3_z = self.rotation.z.to_radians().cos();
        let s3_z = self.rotation.z.to_radians().sin();

        return glam::Mat4::from_cols(
            glam::Vec4::new(
                self.scale.x * (c2_y * c3_z),
                self.scale.x * (s1_x * s2_y * c3_z - c1_x * s3_z),
                self.scale.x * (c1_x * s2_y * c3_z + s1_x * s3_z),
                0.0,
            ),
            glam::Vec4::new(
                self.scale.y * (c2_y * s3_z),
                self.scale.y * (s1_x * s2_y * s3_z + c1_x * c3_z),
                self.scale.y * (c1_x * s2_y * s3_z - s1_x * c3_z),
                0.0,
            ),
            glam::Vec4::new(
                self.scale.z * (-s2_y),
                self.scale.z * (s1_x * c2_y),
                self.scale.z * (c1_x * c2_y),
                0.0,
            ),
            glam::Vec4::new(
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

        return glam::Mat4::from_cols(
            glam::Vec4::new(
                inverse_scale.x * (c1 * c3 + s1 * s2 * s3),
                inverse_scale.x * (c2 * s3),
                inverse_scale.x * (c1 * s2 * s3 - c3 * s1),
                1.0,
            ),
            glam::Vec4::new(
                inverse_scale.y * (c3 * s1 * s2 - c1 * s3),
                inverse_scale.y * (c2 * c3),
                inverse_scale.y * (c1 * c3 * s2 + s1 * s3),
                1.0,
            ),
            glam::Vec4::new(
                inverse_scale.z * (c2 * s1),
                inverse_scale.z * (-s2),
                inverse_scale.z * (c1 * c2),
                1.0,
            ),
            glam::Vec4::new(1.0, 1.0, 1.0, 1.0),
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
