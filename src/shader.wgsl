// todo: tiles start white

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
    
    // todo: this should not be hardcoded
    let col = index % uniform_test.tiles_x;
    let row = index / uniform_test.tiles_x;
    let x = model.position.x;
    let y = model.position.y;

    let SCALE_LEFT_CORNER = 0.8;
    let GAP = 1.25;
    let SCALE_CENTER = 0.25;
    out.clip_position = vec4<f32>(
        (x + f32(col)*GAP)*SCALE_CENTER - SCALE_LEFT_CORNER,
        (y - f32(row)*GAP)*SCALE_CENTER + SCALE_LEFT_CORNER,
        0.0,
        1.0,
    );   

    // todo: low and high has opposite meaning in my code
    out.color = smoothstep(
        instance_color_range.high + vec3(0.01), 
        instance_color_range.low,
        vec3(instance_strength.value)
    );
 
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
