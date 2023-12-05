use wgpu::VertexAttribute;

use super::game_object::Component;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[allow(dead_code)]
pub struct PushValues {
    model_matrix: [[f32; 4]; 4],
    normal_matrix: [[f32; 3]; 3],
}

impl Default for PushValues {
    fn default() -> Self {
        Self {
            model_matrix: [[0.0; 4]; 4],
            normal_matrix: [[0.0; 3]; 3],
        }
    }
}

pub struct Transform {
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Vec3,
    values: PushValues,
}

impl Transform {
    pub fn update_mat4(&mut self) {
        let c1_x = self.rotation.x.cos();
        let s1_x = self.rotation.x.sin();
        let c2_y = self.rotation.y.cos();
        let s2_y = self.rotation.y.sin();
        let c3_z = self.rotation.z.cos(); // Use the absolute value of Z-axis rotation
        let s3_z = self.rotation.z.sin(); // Use the absolute value of Z-axis rotation

        self.values.model_matrix = glam::Mat4::from_cols(
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
        )
        .to_cols_array_2d();
    }

    pub fn update_normal_matrix(&mut self) {
        let c3: f32 = self.rotation.z.cos();
        let s3: f32 = self.rotation.z.sin();
        let c2: f32 = self.rotation.x.cos();
        let s2: f32 = self.rotation.x.sin();
        let c1: f32 = self.rotation.y.cos();
        let s1: f32 = self.rotation.y.sin();
        let inverse_scale: glam::Vec3 = 1.0 / self.scale;

        self.values.normal_matrix = glam::Mat3::from_cols(
            glam::Vec3::new(
                inverse_scale.x * (c1 * c3 + s1 * s2 * s3),
                inverse_scale.x * (c2 * s3),
                inverse_scale.x * (c1 * s2 * s3 - c3 * s1),
            ),
            glam::Vec3::new(
                inverse_scale.y * (c3 * s1 * s2 - c1 * s3),
                inverse_scale.y * (c2 * c3),
                inverse_scale.y * (c1 * c3 * s2 + s1 * s3),
            ),
            glam::Vec3::new(
                inverse_scale.z * (c2 * s1),
                inverse_scale.z * (-s2),
                inverse_scale.z * (c1 * c2),
            ),
        )
        .to_cols_array_2d();
    }

    pub fn to_constant(&self) -> (PushValues, wgpu::VertexBufferLayout) {
        return (
            self.values,
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<PushValues>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &[
                    VertexAttribute {
                        offset: 0,
                        shader_location: 5,
                        format: wgpu::VertexFormat::Float32x4,
                    },
                    VertexAttribute {
                        offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                        shader_location: 6,
                        format: wgpu::VertexFormat::Float32x4,
                    },
                    VertexAttribute {
                        offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                        shader_location: 7,
                        format: wgpu::VertexFormat::Float32x4,
                    },
                    VertexAttribute {
                        offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                        shader_location: 8,
                        format: wgpu::VertexFormat::Float32x4,
                    },
                    VertexAttribute {
                        offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                        shader_location: 9,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                    VertexAttribute {
                        offset: std::mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                        shader_location: 10,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                    VertexAttribute {
                        offset: std::mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                        shader_location: 11,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                ],
            },
        );
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: glam::Vec3::default(),
            scale: glam::Vec3::default(),
            rotation: glam::Vec3::default(),
            values: PushValues::default(),
        }
    }
}

impl Component for Transform {}
