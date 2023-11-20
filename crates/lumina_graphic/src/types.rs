#[derive(Debug)]
pub enum LuminaShaderType{
    INT(i32),
    UINT(u32),
    FLOAT(f32),
    BOOL(bool),
    BVEC2(glam::BVec2),
    BVEC3(glam::BVec3),
    BVEC4(glam::BVec4),
    IVEC2(glam::IVec2),
    IVEC3(glam::IVec3),
    IVEC4(glam::IVec4),
    UVEC2(glam::UVec2),
    UVEC3(glam::UVec3),
    UVEC4(glam::UVec4),
    VEC2(glam::Vec2),
    VEC3(glam::Vec3),
    VEC4(glam::Vec4),
    MAT2(glam::Mat2),
    MAT3(glam::Mat3),
    MAT4(glam::Mat4),
}

impl LuminaShaderType {
    pub fn to_ne_bytes(&self,buffer:&mut Vec<u8>){
        match self{
            LuminaShaderType::INT(value) => {
                buffer.extend_from_slice(&value.to_ne_bytes())
            }

            LuminaShaderType::UINT(value) => {
                buffer.extend_from_slice(&value.to_ne_bytes())

            }

            LuminaShaderType::BOOL(value) => {
                let num_byte:u8 = if *value {1} else {0};
                buffer.extend_from_slice(&num_byte.to_ne_bytes());
            }

            LuminaShaderType::FLOAT(value) => {
                buffer.extend_from_slice(&value.to_ne_bytes())

            }

            LuminaShaderType::BVEC2(value) => {
                buffer.extend_from_slice(&if value.x {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.y {1u8} else {0u8}.to_ne_bytes())
            }

            LuminaShaderType::BVEC3(value) => {
                buffer.extend_from_slice(&if value.x {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.y {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.z {1u8} else {0u8}.to_ne_bytes()); 
            }

            LuminaShaderType::BVEC4(value) => {
                buffer.extend_from_slice(&if value.x {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.y {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.z {1u8} else {0u8}.to_ne_bytes()); 
                buffer.extend_from_slice(&if value.w {1u8} else {0u8}.to_ne_bytes());
            }

            LuminaShaderType::IVEC2(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
            }

            LuminaShaderType::IVEC3(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
            }

            LuminaShaderType::IVEC4(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
                buffer.extend_from_slice(&value.w.to_ne_bytes());
            }

            LuminaShaderType::UVEC2(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
            }

            LuminaShaderType::UVEC3(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
            }

            LuminaShaderType::UVEC4(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
                buffer.extend_from_slice(&value.w.to_ne_bytes());
            }

            LuminaShaderType::VEC2(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
            }

            LuminaShaderType::VEC3(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
            }

            LuminaShaderType::VEC4(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
                buffer.extend_from_slice(&value.w.to_ne_bytes());
            }

            LuminaShaderType::MAT2(value) => {
                let mut slice:[f32;4] = [0.0;4];
                value.write_cols_to_slice(&mut slice);
                for value in slice {
                    buffer.extend_from_slice(&value.to_ne_bytes());
                }
            }

            LuminaShaderType::MAT3(value) => {
                let mut slice:[f32;9] = [0.0;9];
                value.write_cols_to_slice(&mut slice);
                for value in slice {
                    buffer.extend_from_slice(&value.to_ne_bytes());
                }
            }

            LuminaShaderType::MAT4(value) => {
                let mut slice:[f32;16] = [0.0;16];
                value.write_cols_to_slice(&mut slice);
                for value in slice {
                    buffer.extend_from_slice(&value.to_ne_bytes());
                }
            }
        }
    }
}

pub trait LuminaShaderTypeConverter{
    fn to_primitive_value(value:LuminaShaderType) -> Self;
}

impl LuminaShaderTypeConverter for i32 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::INT(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl LuminaShaderTypeConverter for glam::IVec2 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::IVEC2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::IVec3 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::IVEC3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::IVec4 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::IVEC4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl LuminaShaderTypeConverter for u32 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::UINT(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl LuminaShaderTypeConverter for glam::UVec2 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::UVEC2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl LuminaShaderTypeConverter for glam::UVec3 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::UVEC3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl LuminaShaderTypeConverter for glam::UVec4 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::UVEC4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl LuminaShaderTypeConverter for f32 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::FLOAT(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Vec2 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::VEC2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl LuminaShaderTypeConverter for glam::Vec3 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::VEC3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Vec4 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::VEC4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}



impl LuminaShaderTypeConverter for bool {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::BOOL(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::BVec2 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::BVEC2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::BVec3 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::BVEC3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::BVec4 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::BVEC4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Mat2 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::MAT2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Mat3 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::MAT3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl LuminaShaderTypeConverter for glam::Mat4 {
    fn to_primitive_value(value:LuminaShaderType) -> Self {
        if let LuminaShaderType::MAT4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

 
