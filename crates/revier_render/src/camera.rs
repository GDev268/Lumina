pub struct Camera {
    projection_matrix: glam::Mat4,
    view_matrix: glam::Mat4,
    inverse_view_matrix: glam::Mat4,
}

impl Camera {
    pub fn new() -> Self {
        return Self {
            projection_matrix: glam::Mat4::default(),
            view_matrix: glam::Mat4::default(),
            inverse_view_matrix: glam::Mat4::default(),
        };
    }

    pub fn set_orthographic_projection(
        &mut self,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) {
        self.projection_matrix = glam::Mat4::default();
        self.projection_matrix.x_axis.x = 2.0 / (right - left);
        self.projection_matrix.y_axis.y = 2.0 / (bottom - top);
        self.projection_matrix.z_axis.z = 1.0 / (far - near);
        self.projection_matrix.w_axis.x = -(right + left) / (right - left);
        self.projection_matrix.w_axis.y = -(bottom + top) / (bottom - top);
        self.projection_matrix.w_axis.z = -near / (far - near);
    }

    pub fn set_perspective_projection(&mut self, fovy: f32, aspect: f32, near: f32, far: f32) {
        assert!((aspect - f32::EPSILON) > 0.0);

        let tan_half_fovy = (fovy / 2.0).tan();
        self.projection_matrix = glam::Mat4::default();
        self.projection_matrix.x_axis.x = 1.0 / (aspect * tan_half_fovy);
        self.projection_matrix.y_axis.y = 1.0 / (tan_half_fovy);
        self.projection_matrix.z_axis.z = far / (far - near);
        self.projection_matrix.z_axis.w = 1.0;
        self.projection_matrix.w_axis.z = -(far * near) / (far - near);
    }

    pub fn set_view_direction(
        &mut self,
        position: glam::Vec3,
        direction: glam::Vec3,
        up: Option<glam::Vec3>,
    ) {
        let up: glam::Vec3 = if up.is_none() {
            glam::Vec3::ONE
        } else {
            up.unwrap()
        };

        let w: glam::Vec3 = direction.normalize();
        let u = w.cross(up);
        let v = w.cross(u);

        self.view_matrix = glam::Mat4::from_cols(
            u.extend(0.0),
            v.extend(0.0),
            w.extend(0.0),
            glam::vec4(-u.dot(position), -v.dot(position), -w.dot(position), 0.0),
        );

        self.inverse_view_matrix = glam::Mat4::from_cols(
            u.extend(0.0),
            v.extend(0.0),
            w.extend(0.0),
            glam::vec4(position.x, position.y, position.z, 0.0),
        );
    }

    pub fn set_view_target(
        &mut self,
        position: glam::Vec3,
        target: glam::Vec3,
        up: Option<glam::Vec3>,
    ) {
        self.set_view_direction(position, target - position, up);
    }
    pub fn set_view_yxz(&mut self, translation: glam::Vec3, rotation: glam::Vec3) {
        let c1 = rotation.x.cos();
        let s1 = rotation.x.sin();
        let c2 = rotation.y.cos();
        let s2 = rotation.y.sin();
        let c3 = rotation.z.cos();
        let s3 = rotation.z.sin();
    
        let u = glam::Vec3::new(c1 * c3 + s1 * s2 * s3, -s3 * c2, c3 * s1 - c1 * s2 * s3);
        let v = glam::Vec3::new(c1 * s2, c2, s1 * s2);
        let w = glam::Vec3::new(s1 * c3 - c1 * s2 * s3, s3 * c2, -c1 * c3 - s1 * s2 * s3);
    
        // Adjust the Z-component of the position vector for movement along the negative Z-axis.
        let adjusted_position = glam::Vec3::new(
            translation.x,
            translation.y,
            translation.z, // No adjustment needed for Z-axis
        );
    
        self.view_matrix = glam::Mat4::from_cols(
            u.extend(0.0),
            v.extend(0.0),
            w.extend(0.0),
            glam::Vec4::new(
                -u.dot(adjusted_position),
                -v.dot(adjusted_position),
                -w.dot(adjusted_position),
                0.0,
            ),
        );
    
        self.inverse_view_matrix = glam::Mat4::from_cols(
            u.extend(0.0),
            v.extend(0.0),
            w.extend(0.0),
            adjusted_position.extend(1.0),
        );
    }
    
    
    

    pub fn get_projection(&self) -> glam::Mat4 {
        return self.projection_matrix;
    }

    pub fn get_view(&self) -> glam::Mat4 {
        return self.view_matrix;
    }

    pub fn get_inverse_view(&self) -> glam::Mat4 {
        return self.inverse_view_matrix;
    }

    pub fn get_position(&self) -> glam::Vec3 {
        return glam::Vec3 {
            x: self.inverse_view_matrix.w_axis.x,
            y: self.inverse_view_matrix.w_axis.y,
            z: self.inverse_view_matrix.w_axis.z,
        };
    }
}
