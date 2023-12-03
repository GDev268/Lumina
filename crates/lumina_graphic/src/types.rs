use lumina_data::type_padding::{IntPadding, FloatPadding, Vec2Padding, Vec3Padding, Vec4Padding, Mat2Padding, Mat3Padding, Mat4Padding};

#[derive(Debug)]
pub enum LuminaShaderType {
    INT(IntPadding),
    UINT(u32),
    FLOAT(FloatPadding),
    BOOL(bool),
    BVEC2([bool; 2]),
    BVEC3([bool; 3]),
    BVEC4([bool; 4]),
    IVEC2([u32; 2]),
    IVEC3([u32; 3]),
    IVEC4([u32; 4]),
    UVEC2([u32; 2]),
    UVEC3([u32; 3]),
    UVEC4([u32; 4]),
    VEC2(Vec2Padding),
    VEC3(Vec3Padding),
    VEC4(Vec4Padding),
    MAT2(Mat2Padding),
    MAT3(Mat3Padding),
    MAT4(Mat4Padding),
}

impl LuminaShaderType {
    pub fn to_ne_bytes(values: Vec<LuminaShaderType>) -> Vec<u8> {
        let mut buffer:Vec<u8> = Vec::new();
        for value in values {
            match value {
                LuminaShaderType::INT(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::BOOL(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::UINT(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::FLOAT(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::VEC2(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::VEC3(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::VEC4(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::UVEC2(value) => buffer.extend_from_slice(bytemuck::cast_slice(&value)),
                LuminaShaderType::UVEC3(value) => buffer.extend_from_slice(bytemuck::cast_slice(&value)),
                LuminaShaderType::UVEC4(value) => buffer.extend_from_slice(bytemuck::cast_slice(&value)),
                LuminaShaderType::BVEC2(value) => buffer.extend_from_slice(bytemuck::cast_slice(&value)),
                LuminaShaderType::BVEC3(value) => buffer.extend_from_slice(bytemuck::cast_slice(&value)),
                LuminaShaderType::BVEC4(value) => buffer.extend_from_slice(bytemuck::cast_slice(&value)),
                LuminaShaderType::IVEC2(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::IVEC3(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::IVEC4(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::MAT2(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::MAT3(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                LuminaShaderType::MAT4(value) => buffer.extend_from_slice(bytemuck::cast_slice(&[value])),
                _ => buffer.extend_from_slice(&[])
            };
        }

        return buffer;
    }
}

/*pub trait LuminaShaderTypeConverter {
    fn to_primitive_value(value: LuminaShaderType) -> Self;
}

impl LuminaShaderTypeConverter for i32 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::INT(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::IVec2 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::IVEC2(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::IVec3 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::IVEC3(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::IVec4 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::IVEC4(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for u32 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::UINT(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::UVec2 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::UVEC2(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::UVec3 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::UVEC3(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::UVec4 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::UVEC4(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for f32 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::FLOAT(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Vec2 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::VEC2(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Vec3 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::VEC3(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Vec4 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::VEC4(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for bool {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::BOOL(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::BVec2 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::BVEC2(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::BVec3 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::BVEC3(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::BVec4 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::BVEC4(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Mat2 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::MAT2(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Mat3 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::MAT3(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Mat4 {
    fn to_primitive_value(value: LuminaShaderType) -> Self {
        if let LuminaShaderType::MAT4(v) = value {
            v
        } else {
            panic!("Error: Failed to get the value")
        }
    }
}*/
