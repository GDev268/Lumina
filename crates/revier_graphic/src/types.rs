#[derive(Debug)]
pub enum RevierShaderType{
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

impl RevierShaderType {
    pub fn to_ne_bytes(&self,buffer:&mut Vec<u8>){
        match self{
            RevierShaderType::INT(value) => {
                buffer.extend_from_slice(&value.to_ne_bytes())
            }

            RevierShaderType::UINT(value) => {
                buffer.extend_from_slice(&value.to_ne_bytes())

            }

            RevierShaderType::BOOL(value) => {
                let num_byte:u8 = if *value {1} else {0};
                buffer.extend_from_slice(&num_byte.to_ne_bytes());
            }

            RevierShaderType::FLOAT(value) => {
                buffer.extend_from_slice(&value.to_ne_bytes())

            }

            RevierShaderType::BVEC2(value) => {
                buffer.extend_from_slice(&if value.x {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.y {1u8} else {0u8}.to_ne_bytes())
            }

            RevierShaderType::BVEC3(value) => {
                buffer.extend_from_slice(&if value.x {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.y {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.z {1u8} else {0u8}.to_ne_bytes()); 
            }

            RevierShaderType::BVEC4(value) => {
                buffer.extend_from_slice(&if value.x {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.y {1u8} else {0u8}.to_ne_bytes());
                buffer.extend_from_slice(&if value.z {1u8} else {0u8}.to_ne_bytes()); 
                buffer.extend_from_slice(&if value.w {1u8} else {0u8}.to_ne_bytes());
            }

            RevierShaderType::IVEC2(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
            }

            RevierShaderType::IVEC3(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
            }

            RevierShaderType::IVEC4(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
                buffer.extend_from_slice(&value.w.to_ne_bytes());
            }

            RevierShaderType::UVEC2(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
            }

            RevierShaderType::UVEC3(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
            }

            RevierShaderType::UVEC4(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
                buffer.extend_from_slice(&value.w.to_ne_bytes());
            }

            RevierShaderType::VEC2(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
            }

            RevierShaderType::VEC3(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
            }

            RevierShaderType::VEC4(value) => {
                buffer.extend_from_slice(&value.x.to_ne_bytes());
                buffer.extend_from_slice(&value.y.to_ne_bytes());
                buffer.extend_from_slice(&value.z.to_ne_bytes());
                buffer.extend_from_slice(&value.w.to_ne_bytes());
            }

            RevierShaderType::MAT2(value) => {
                let mut slice:[f32;4] = [0.0;4];
                value.write_cols_to_slice(&mut slice);
                for value in slice {
                    buffer.extend_from_slice(&value.to_ne_bytes());
                }
            }

            RevierShaderType::MAT3(value) => {
                let mut slice:[f32;9] = [0.0;9];
                value.write_cols_to_slice(&mut slice);
                for value in slice {
                    buffer.extend_from_slice(&value.to_ne_bytes());
                }
            }

            RevierShaderType::MAT4(value) => {
                let mut slice:[f32;16] = [0.0;16];
                value.write_cols_to_slice(&mut slice);
                for value in slice {
                    buffer.extend_from_slice(&value.to_ne_bytes());
                }
            }
        }
    }
}

pub trait RevierShaderTypeConverter{
    fn to_primitive_value(value:RevierShaderType) -> Self;
}

impl RevierShaderTypeConverter for i32 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::INT(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl RevierShaderTypeConverter for glam::IVec2 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::IVEC2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::IVec3 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::IVEC3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::IVec4 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::IVEC4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl RevierShaderTypeConverter for u32 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::UINT(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl RevierShaderTypeConverter for glam::UVec2 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::UVEC2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl RevierShaderTypeConverter for glam::UVec3 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::UVEC3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl RevierShaderTypeConverter for glam::UVec4 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::UVEC4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl RevierShaderTypeConverter for f32 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::FLOAT(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::Vec2 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::VEC2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}

impl RevierShaderTypeConverter for glam::Vec3 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::VEC3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::Vec4 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::VEC4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

}



impl RevierShaderTypeConverter for bool {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::BOOL(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::BVec2 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::BVEC2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::BVec3 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::BVEC3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::BVec4 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::BVEC4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::Mat2 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::MAT2(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::Mat3 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::MAT3(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

impl RevierShaderTypeConverter for glam::Mat4 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::MAT4(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}

 
