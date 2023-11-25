struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) normalPos: vec2<f32>,
    @location(2) uv: vec2<f32>
};

struct VertexOutput{
    @location(0) fragPos: vec3<f32>,
    @location(1) normal: vec3<f32>
}

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {

}