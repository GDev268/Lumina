/*pub struct Camera {
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

        let (sin_fov, cos_fov) = (0.5 * fovy).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect;
        let r = far / (far - near);
        self.projection_matrix = glam::Mat4::from_cols(
            glam::vec4(w, 0.0, 0.0, 0.0),
            glam::vec4(0.0, h, 0.0, 0.0),
            glam::vec4(0.0, 0.0, r, 1.0),
            glam::vec4(0.0, 0.0, -r * near, 0.0),
        )
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
        let c3 = rotation.z.cos();
        let s3 = rotation.z.sin();
        let c2 = rotation.x.cos();
        let s2 = rotation.x.sin();
        let c1 = rotation.y.cos();
        let s1 = rotation.y.sin();

        let u = glam::Vec3::new(c1 * c3 + s1 * s2 * s3, c2 * s3, c1 * s2 * s3 - c3 * s1);
        let v = glam::Vec3::new(c3 * s1 * s2 - c1 * s3, c2 * c3, c1 * c3 * s2 + s1 * s3);
        let w = glam::Vec3::new(c2 * s1, -s2, c1 * c2);

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
                1.0,
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
*/