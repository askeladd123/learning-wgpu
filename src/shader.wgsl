struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct InstanceStrength{
    @location(5) value: f32,
}

struct InstanceColorRange{
    @location(6) high: vec3f,
    @location(7) low: vec3f,
}

struct UniformTest{
    tiles_x: u32,
    gap: f32,
    margin: f32,
    speed: f32,
    mouse_speed: f32,
    mouse: vec2f,
}
@group(0) @binding(0)
var<uniform> uniform_test: UniformTest;

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) index: u32,
    instance_strength: InstanceStrength,
    instance_color_range: InstanceColorRange,
) -> VertexOutput {
    var out: VertexOutput;
    // out.color = vec3f(model.position.x * 2.0, 1.0 - model.position.y * 2.0, model.position.z);
    
    // todo: this cannot be hardcoded
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

    out.color = smoothstep(instance_color_range.low, instance_color_range.high, 
    vec3<f32>(
        instance_strength.value,
        instance_strength.value,
        instance_strength.value,
    ));

    out.color = smoothstep(
        vec3<f32>(1.0, 1.0, 1.0),
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(
            instance_strength.value,
            instance_strength.value,
            instance_strength.value,
        )
    );

    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
