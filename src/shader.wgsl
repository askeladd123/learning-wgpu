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
    // out.color = vec3f(model.position.x * 2.0, 1.0 - model.position.y * 2.0, model.position.z);
    
    let tiles_x = u32(6);
    let col = index % tiles_x;
    let row = index / tiles_x;
    let x = model.position.x;
    let y = model.position.y;

    out.clip_position = vec4<f32>(
        (x + f32(col)*1.25)*0.25 - 0.8,
        (y - f32(row)*1.25)*0.25 + 0.8,
        0.0,
        1.0,
    );   
    out.color = vec3f(model.color);
    // out.clip_position = vec4<f32>(
        // (model.position.x + f32(index)*1.25)*0.25 - 1.0, model.position.y, model.position.z, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
