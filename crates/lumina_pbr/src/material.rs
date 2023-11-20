use lumina_graphic::shader::Shader;
use lumina_object::game_object::Component;

pub struct Material {
    pub ambient:glam::Vec3,
    pub diffuse:glam::Vec3,
    pub specular:glam::Vec3,
    pub shininess:f32
}

impl Default for Material {
    fn default() -> Self {
        Self { 
            ambient: glam::Vec3::default(), 
            diffuse: glam::Vec3::default(), 
            specular: glam::Vec3::default(), 
            shininess: 0.0
        }
    }
}

impl Material {
    pub fn new(ambient:glam::Vec3,diffuse:glam::Vec3,specular:glam::Vec3,shininess:f32) -> Self {
        Self { ambient, diffuse, specular, shininess }
    }

    pub fn mix(material1:Material,material2:Material,percentage:f32) -> Material {
        Material { 
            ambient: material1.ambient * percentage + material2.ambient * (1.0 - percentage), 
            diffuse: material1.diffuse * percentage + material2.diffuse * (1.0 - percentage), 
            specular: material1.specular * percentage + material2.specular * (1.0 - percentage), 
            shininess: material1.shininess * percentage + material2.shininess * (1.0 - percentage) 
        }
    }

    pub fn update_shader(&self,shader:&mut Shader) {
        shader.change_uniform_vec3("ObjectProperties.material.ambient",self.ambient).unwrap();
        shader.change_uniform_vec3("ObjectProperties.material.diffuse",self.ambient).unwrap();
        shader.change_uniform_vec3("ObjectProperties.material.specular",self.specular).unwrap();
        shader.change_uniform_1f("ObjectProperties.material.shininess",self.shininess).unwrap();        
    }

    pub const TEST:Material = Material{
        ambient: glam::vec3(1.0, 1.0, 1.0), 
        diffuse: glam::vec3(0.75164, 0.60648, 0.22648), 
        specular: glam::vec3(0.628281, 0.555802, 0.366065), 
        shininess: 0.4
    };
}

impl Component for Material {}