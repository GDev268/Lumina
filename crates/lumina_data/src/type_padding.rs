// Import necessary crates
use bytemuck::{Pod, Zeroable};
use std::fmt::Debug;

// Scalar Types
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct IntPadding {
    pub int_val: i32,
    pub padding: [u32; 3], // Adjust padding to ensure overall size is a multiple of 16 bytes
}

impl Default for IntPadding {
    fn default() -> Self {
        IntPadding {
            int_val: 0,
            padding: [0; 3],
        }
    }
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct FloatPadding {
    pub float_val: f32,
    pub padding: [u32; 3], // Adjust padding to ensure overall size is a multiple of 16 bytes
}

impl Default for FloatPadding {
    fn default() -> Self {
        FloatPadding {
            float_val: 0.0,
            padding: [0; 3],
        }
    }
}

// Vector Types
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vec2Padding {
    pub vec2_val: [f32; 2],
    pub padding: [u32; 2],
}

impl Default for Vec2Padding {
    fn default() -> Self {
        Vec2Padding {
            vec2_val: [0.0, 0.0],
            padding: [0; 2],
        }
    }
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vec3Padding {
    pub vec3_val: [f32; 3],
    pub padding: [u32; 1],
}

impl Default for Vec3Padding {
    fn default() -> Self {
        Vec3Padding {
            vec3_val: [0.0, 0.0, 0.0],
            padding: [0],
        }
    }
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vec4Padding {
    pub vec4_val: [f32; 4],
}

impl Default for Vec4Padding {
    fn default() -> Self {
        Vec4Padding {
            vec4_val: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

// Matrix Types
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Mat2Padding {
    pub mat2_val: [[f32; 2]; 2],
}

impl Default for Mat2Padding {
    fn default() -> Self {
        Mat2Padding {
            mat2_val: [[0.0, 0.0], [0.0, 0.0]],
        }
    }
}

// Matrix Types
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Mat3Padding {
    pub mat3_val: [[f32; 3]; 3],
    pub padding: [u32; 3], // Adjust padding to ensure overall size is a multiple of 16 bytes
}

impl Default for Mat3Padding {
    fn default() -> Self {
        Mat3Padding {
            mat3_val: [[0.0; 3]; 3],
            padding: [0; 3],
        }
    }
}


#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Mat4Padding {
    pub mat4_val: [[f32; 4]; 4],
}

impl Default for Mat4Padding {
    fn default() -> Self {
        Mat4Padding {
            mat4_val: [[0.0; 4]; 4],
        }
    }
}
