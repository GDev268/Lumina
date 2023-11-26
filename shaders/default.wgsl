struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) normal_pos: vec3<f32>,
    @location(2) uv: vec2<f32>
};

struct VertexOutput{
    @builtin(position) vert_pos: vec4<f32>,
    @location(0) frag_pos: vec3<f32>,
    @location(1) normal: vec3<f32>
}

struct ShaderProperties{
    @location(4) model_matrix_0: vec4<f32>,
    @location(5) model_matrix_1: vec4<f32>,
    @location(6) model_matrix_2: vec4<f32>,
    @location(7) model_matrix_3: vec4<f32>,

    @location(8) normal_matrix_0: vec4<f32>, 
    @location(9) normal_matrix_1: vec4<f32>,
    @location(10) normal_matrix_2: vec4<f32>, 
    @location(11) normal_matrix_3: vec4<f32>

}

@vertex
fn vs_main(in: VertexInput,props:ShaderProperties) -> VertexOutput {
    var out:VertexOutput;
    let model_mat:mat4x4<f32> = mat4x4(props.model_matrix_0,props.model_matrix_1,props.model_matrix_2,props.model_matrix_3);

    out.vert_pos = model_mat * vec4<f32>(in.position,1.0);
    out.normal = normalize(mat3x4<f32>(props.normal_matrix_0,props.normal_matrix_1,props.normal_matrix_2) * in.normal_pos).xyz;
    out.frag_pos = (model_mat * vec4<f32>(in.position,1.0)).xyz;

    return out;
}



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0,1.0,1.0,1.0);
} 