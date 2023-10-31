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

pub trait RevierShaderTypeConverter{
    fn to_primitive_value(value:RevierShaderType) -> Self;
    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>);
}

impl RevierShaderTypeConverter for i32 {
    fn to_primitive_value(value:RevierShaderType) -> Self {
        if let RevierShaderType::INT(v) = value {
            v
        }else{
            panic!("Error: Failed to get the value")
        }
    }

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::INT(v) = value {
            buffer.extend_from_slice(&v.to_ne_bytes())
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::IVEC2(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::IVEC3(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
            buffer.extend_from_slice(&v.z.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::IVEC4(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
            buffer.extend_from_slice(&v.z.to_ne_bytes());
            buffer.extend_from_slice(&v.w.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::UINT(v) = value {
            buffer.extend_from_slice(&v.to_ne_bytes())
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::UVEC2(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::UVEC3(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
            buffer.extend_from_slice(&v.z.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::UVEC4(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
            buffer.extend_from_slice(&v.z.to_ne_bytes());
            buffer.extend_from_slice(&v.w.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::FLOAT(v) = value {
            buffer.extend_from_slice(&v.to_ne_bytes())
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::VEC2(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::VEC3(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
            buffer.extend_from_slice(&v.z.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::VEC4(v) = value {
            buffer.extend_from_slice(&v.x.to_ne_bytes());
            buffer.extend_from_slice(&v.y.to_ne_bytes());
            buffer.extend_from_slice(&v.z.to_ne_bytes());
            buffer.extend_from_slice(&v.w.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::BOOL(v) = value {
            let num_byte:u8 = if v {1} else {0};
            buffer.extend_from_slice(&num_byte.to_ne_bytes());
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::BVEC2(v) = value {
            buffer.extend_from_slice(&if v.x {1u8} else {0u8}.to_ne_bytes());
            buffer.extend_from_slice(&if v.y {1u8} else {0u8}.to_ne_bytes()) 
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::BVEC3(v) = value {
            buffer.extend_from_slice(&if v.x {1u8} else {0u8}.to_ne_bytes());
            buffer.extend_from_slice(&if v.y {1u8} else {0u8}.to_ne_bytes());
            buffer.extend_from_slice(&if v.z {1u8} else {0u8}.to_ne_bytes()); 
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

    fn to_ne_bytes(value:RevierShaderType,buffer:&mut Vec<u8>){
        if let RevierShaderType::BVEC4(v) = value {
            buffer.extend_from_slice(&if v.x {1u8} else {0u8}.to_ne_bytes());
            buffer.extend_from_slice(&if v.y {1u8} else {0u8}.to_ne_bytes());
            buffer.extend_from_slice(&if v.z {1u8} else {0u8}.to_ne_bytes());
            buffer.extend_from_slice(&if v.w {1u8} else {0u8}.to_ne_bytes());
        }else{
            panic!("Error: Failed to get the value")
        }
    }
}
