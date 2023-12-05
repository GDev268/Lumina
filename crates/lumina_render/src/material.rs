use wgpu::BindGroupLayoutEntry;

struct Material {
    ambient:glam::Vec3,
    diffuse:glam::Vec3,
    specular:glam::Vec3,
    shininess:f32,
    value: RawMaterial
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RawMaterial {
    ambient:[f32;3],
    diffuse:[f32;3],
    specular:[f32;3],
    shininess:f32
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

    const TEST:Material = Material{
        ambient: glam::vec3(0.24725, 0.1995, 0.0745), 
        diffuse: glam::vec3(0.75164, 0.60648, 0.22648), 
        specular: glam::vec3(0.628281, 0.555802, 0.366065), 
        shininess: 0.4
    };

    pub fn get_uniform<'a>(&self) -> (u32,wgpu::BindGroupLayoutDescriptor<'a>,wgpu::util::BufferInitDescriptor<'a>) {
        return (1,wgpu::BindGroupLayoutDescriptor{
            label: Some("1_Bind_Group"),
            entries: &[
                BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None
                }
            ],
        },wgpu::util::BufferInitDescriptor{
            label: Some("1_Buffer_Descriptor"),
            contents: bytemuck::cast_slice(&[RawMaterial{
                ambient: self.ambient.to_array(),
                diffuse: self.diffuse.to_array(),
                specular: self.specular.to_array(),
                shininess: self.shininess
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }
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