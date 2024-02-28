use lumina_object::game_object::Component;

pub enum CameraDirection {
    NONE,
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Debug,Clone, Copy)]
pub struct Camera {
    projection_matrix: [[f32; 4]; 4],
    view_matrix: [[f32; 4]; 4],
    inverse_view_matrix: [[f32; 4]; 4],
    pub ortho_mode: bool,
    pub fov: f32,
    pub speed: f32,
    pub sensivity: f32,
    aspect_ratio: f32,
    rotation: glam::Vec3,
    translation: glam::Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, ortho_mode: bool) -> Self {
        return Self {
            projection_matrix: [[1.0; 4]; 4],
            view_matrix: [[1.0; 4]; 4],
            inverse_view_matrix: [[1.0; 4]; 4],
            ortho_mode,
            fov: 50.0,
            speed: 10.0,
            sensivity: 2.0,
            aspect_ratio,
            rotation: glam::Vec3::ZERO,
            translation: glam::Vec3::ZERO,
        };
    }

    pub fn create_orthographic_projection(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> glam::Mat4 {
        let mut projection_matrix = [[0.0; 4]; 4];
        projection_matrix[0][0] = 2.0 / (right - left);
        projection_matrix[1][1] = 2.0 / (top - bottom);
        projection_matrix[2][2] = -2.0 / (far - near);
        projection_matrix[3][0] = -(right + left) / (right - left);
        projection_matrix[3][1] = -(top + bottom) / (top - bottom);
        projection_matrix[3][2] = -(far + near) / (far - near);
        projection_matrix[3][3] = 1.0;
    
        glam::Mat4::from_cols_array_2d(&projection_matrix)
    }

    pub fn create_perspective_projection(fovy: f32, aspect: f32, near: f32, far: f32) -> glam::Mat4 {
        assert!((aspect - f32::EPSILON) > 0.0);

        let (sin_fov, cos_fov) = (0.5 * fovy).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect;
        let r = far / (far - near);
        
        let mut projection_matrix = [[1.0;4];4];
        projection_matrix = [[0.0; 4]; 4];
        projection_matrix[0][0] = w;
        projection_matrix[1][1] = h;
        projection_matrix[2][2] = r;
        projection_matrix[2][3] = 1.0;
        projection_matrix[3][2] = -r * near;

        return glam::Mat4::from_cols_array_2d(&projection_matrix);
    }

    pub fn update_position(&mut self, dir: CameraDirection, dt: f32) {
        let velocity = dt * self.speed;

        // Define the movement direction relative to the camera's local space
        let mut move_direction = match dir {
            CameraDirection::NONE => glam::Vec3::ZERO,
            CameraDirection::FORWARD => glam::Vec3::new(0.0, 0.0, 1.0), // Move forward along the negative z-axis
            CameraDirection::BACKWARD => glam::Vec3::new(0.0, 0.0, -1.0), // Move backward along the positive z-axis
            CameraDirection::LEFT => glam::Vec3::new(-1.0, 0.0, 0.0), // Move left along the negative x-axis
            CameraDirection::RIGHT => glam::Vec3::new(1.0, 0.0, 0.0), // Move right along the positive x-axis
            CameraDirection::UP => glam::Vec3::new(0.0, -1.0, 0.0), // Move up along the positive y-axis
            CameraDirection::DOWN => glam::Vec3::new(0.0, 1.0, 0.0), // Move down along the negative y-axis
        };

        // Rotate the movement direction based on the camera's rotation
        let rotation_matrix = -(glam::Mat3::from_rotation_y(self.rotation.y) * glam::Mat3::from_rotation_x(self.rotation.x));
        move_direction = rotation_matrix * move_direction; // Apply rotation to the movement direction

        if move_direction.length_squared() > std::f32::EPSILON {
            move_direction = -move_direction.normalize();
            self.translation += velocity * move_direction;
        }
    }

    pub fn update_direction(&mut self, dx: f64, dy: f64, dt: f32) {
        let mut rotation = glam::Vec3::ZERO;
    
        if dy > 0.0 {
            rotation.x -= dy.abs() as f32;
        } else if dy < 0.0 {
            rotation.x += dy.abs() as f32;
        }
    
        if dx > 0.0 {
            rotation.y += dx.abs() as f32
        } else if dx < 0.0 {
            rotation.y -= dx.abs() as f32
        }
    
        if rotation.dot(rotation) > std::f32::EPSILON {
            self.rotation += self.sensivity * dt * rotation;
        }
    
        self.rotation.x = self.rotation.x.clamp(-1.5, 1.5);
        self.rotation.y = self.rotation.y % (2.0 * std::f32::consts::PI);
    }

    pub fn get_matrix(&self) -> [[f32;4];4] {
        let perspective = if self.ortho_mode {
            Camera::create_orthographic_projection(
                -self.aspect_ratio,
                self.aspect_ratio,
                -1.0,
                1.0,
                0.1,
                1000.0,
            )
        } else {
            Camera::create_perspective_projection(
                self.fov.to_radians(),
                self.aspect_ratio,
                0.1,
                1000.0,
            )
        };
    
        let view = Camera::set_view_yxz(self.translation, self.rotation);
    
        return (perspective * view).to_cols_array_2d();
    }

    pub fn set_view_direction(
        &mut self,
        position: glam::Vec3,
        direction: glam::Vec3,
        up: glam::Vec3,
    ) {
        let w: glam::Vec3 = direction.normalize();
        let u = w.cross(up);
        let v = w.cross(u);

        self.view_matrix = [[1.0; 4]; 4];
        self.view_matrix[0][0] = u.x;
        self.view_matrix[1][0] = u.y;
        self.view_matrix[2][0] = u.z;
        self.view_matrix[0][1] = v.x;
        self.view_matrix[1][1] = v.y;
        self.view_matrix[2][1] = v.z;
        self.view_matrix[0][2] = w.x;
        self.view_matrix[1][2] = w.y;
        self.view_matrix[2][2] = w.z;
        self.view_matrix[3][0] = -u.dot(position);
        self.view_matrix[3][1] = -v.dot(position);
        self.view_matrix[3][2] = -w.dot(position);

        self.inverse_view_matrix = [[1.0; 4]; 4];
        self.inverse_view_matrix[0][0] = u.x;
        self.inverse_view_matrix[1][0] = u.y;
        self.inverse_view_matrix[2][0] = u.z;
        self.inverse_view_matrix[0][1] = v.x;
        self.inverse_view_matrix[1][1] = v.y;
        self.inverse_view_matrix[2][1] = v.z;
        self.inverse_view_matrix[0][2] = w.x;
        self.inverse_view_matrix[1][2] = w.y;
        self.inverse_view_matrix[2][2] = w.z;
        self.inverse_view_matrix[3][0] = position.x;
        self.inverse_view_matrix[3][1] = position.y;
        self.inverse_view_matrix[3][2] = position.z;
    }

    pub fn set_view_target(
        &mut self,
        position: glam::Vec3,
        target: glam::Vec3,
        up: glam::Vec3,
    ) {
        self.set_view_direction(position, target - position, up);
    }

    pub fn set_view_yxz(position: glam::Vec3, rotation: glam::Vec3) -> glam::Mat4 {
        let c3 = rotation.z.cos();
        let s3 = rotation.z.sin();
        let c2 = rotation.x.cos();
        let s2 = rotation.x.sin();
        let c1 = rotation.y.cos();
        let s1 = rotation.y.sin();

        let u = glam::Vec3::new(c1 * c3 + s1 * s2 * s3, c2 * s3, c1 * s2 * s3 - c3 * s1);
        let v = glam::Vec3::new(c3 * s1 * s2 - c1 * s3, c2 * c3, c1 * c3 * s2 + s1 * s3);
        let w = glam::Vec3::new(c2 * s1, -s2, c1 * c2);

        let adjusted_position = glam::Vec3::new(position.x, position.y, position.z);


        let mut view_matrix: [[f32; 4]; 4] = [[0.0; 4]; 4];
        view_matrix[0][0] = u.x;
        view_matrix[1][0] = u.y;
        view_matrix[2][0] = u.z;
        view_matrix[0][1] = v.x;
        view_matrix[1][1] = v.y;
        view_matrix[2][1] = v.z;
        view_matrix[0][2] = w.x;
        view_matrix[1][2] = w.y;
        view_matrix[2][2] = w.z;
        view_matrix[3][0] = -u.dot(adjusted_position);
        view_matrix[3][1] = -v.dot(adjusted_position);
        view_matrix[3][2] = -w.dot(adjusted_position);
        view_matrix[3][3] = 1.0;

        return glam::Mat4::from_cols_array_2d(&view_matrix);
    }

    pub fn get_inverse_view(&self) -> glam::Mat4 {
        return glam::Mat4::from_cols_array_2d(&self.inverse_view_matrix);
    }

    pub fn get_direction(&self) -> glam::Vec3 {
        return glam::Vec3 {
            x: self.rotation.y.sin(),
            y: 0.0,
            z: self.rotation.y.cos(),
        };
    }

    pub fn get_position(&self) -> glam::Vec3 {
        return glam::Vec3 {
            x: self.translation.x,
            y: self.translation.y,
            z: self.translation.z,
        };
    }
}
