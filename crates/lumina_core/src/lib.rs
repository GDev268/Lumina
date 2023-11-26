pub mod device;
pub mod window;
pub mod fps_manager;


#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32;3],
    pub normal: [f32;3],
    pub uv: [f32;2],
}

impl Vertex {
    const ATTRIBUTES:[wgpu::VertexAttribute;3] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];

    pub fn description<'a>() -> wgpu::VertexBufferLayout<'a> {
        return wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES
        }
    } 
}