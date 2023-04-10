// TODO: tiles start white

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
    
    let nx = f32(uniform_test.tiles_x);
    // let nx = 4.0;
    let col = f32(index % u32(nx));
    let row = f32(index / u32(nx));
    let x = model.position.x;
    let y = model.position.y;
    
    // let w = 2.0 / f32(nx);
    // out.clip_position.x = 
    //     x * 2.0 * (1.0 / nx) + 
    //     row * 2.0 * (1.0 / nx) - 
    //     (1.0 / nx) * 2.0 * (nx - 1.0) * 0.5;
    // out.clip_position.x = 
        // y * 2.0 * (1.0 / nx) + 
        // col * 2.0 * (1.0 / nx) - 
        // (1.0 / nx) * 2.0 * (nx - 1.0) * 0.5;

    out.clip_position.x = (x + row - (nx - 1.0) * 0.5) * 2.0 / nx;
    out.clip_position.y = (y + col - (nx - 1.0) * 0.5) * 2.0 / nx;
    out.clip_position.w = 1.0;
    
    // FIXME: low and high has opposite meaning in my code
    // FIXME: white to grey transition turns black
    // low field cannot be zero
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
