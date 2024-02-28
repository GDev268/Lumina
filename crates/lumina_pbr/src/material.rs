use lumina_core::texture::Texture;
use lumina_graphic::shader::Shader;
use lumina_object::game_object::Component;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct MaterialRaw {
    ambient: [f32; 3],
    _padding1: u32,
    diffuse: [f32; 3],
    _padding2: u32,
    specular: [f32; 3],
    shininess: f32,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct MaterialInfo {
    material: MaterialRaw,
    view_pos: [f32; 3],
}

#[derive(Debug)]
pub struct Material {
    pub ambient: glam::Vec3,
    pub ambient_texture: Texture,
    pub diffuse: glam::Vec3,
    pub metallic: glam::Vec3,
    pub metallic_texture: Texture,
    pub normal_texture: Texture,

    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: glam::Vec3::default(),
            diffuse: glam::Vec3::default(),
            metallic: glam::Vec3::default(),
            shininess: 0.0,
            ambient_texture: Texture::new_raw(""),
            metallic_texture: Texture::new_raw(""),
            normal_texture: Texture::new_raw(""),
        }
    }
}

impl Material {
    pub fn new(
        ambient: glam::Vec3,
        diffuse: glam::Vec3,
        metallic: glam::Vec3,
        shininess: f32,
    ) -> Self {
        Self {
            ambient,
            diffuse,
            metallic,
            shininess,
            ambient_texture: Texture::new_raw(""),
            metallic_texture: Texture::new_raw(""),
            normal_texture: Texture::new_raw(""),
        }
    }

    pub fn mix(material1: Material, material2: Material, percentage: f32) -> Material {
        Material {
            ambient: material1.ambient * percentage + material2.ambient * (1.0 - percentage),
            diffuse: material1.diffuse * percentage + material2.diffuse * (1.0 - percentage),
            metallic: material1.metallic * percentage + material2.metallic * (1.0 - percentage),
            shininess: material1.shininess * percentage + material2.shininess * (1.0 - percentage),
            ambient_texture: Texture::new_raw(""),
            metallic_texture: Texture::new_raw(""),
            normal_texture: Texture::new_raw(""),
        }
    }

    pub fn get_material_info(&self, view_pos: [f32; 3]) -> MaterialInfo {
        MaterialInfo {
            material: MaterialRaw {
                ambient: self.ambient.to_array(),
                _padding1: 0,
                diffuse: self.diffuse.to_array(),
                _padding2: 0,
                specular: self.metallic.to_array(),
                shininess: self.shininess,
            },
            view_pos,
        }
    }
}
