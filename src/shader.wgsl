// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = vec3f(model.position.x * 2.0, 1.0 - model.position.y * 2.0, model.position.z);
    out.clip_position = vec4<f32>(model.position.x + f32(index), model.position.y, model.position.z, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
