struct ResolutionUniform {
    width: f32,
    height: f32,
    cell_size: f32, // minimum binding size should be 16 bytes
    _offset2: f32,
}

@group(0) @binding(0)
var <uniform> resolution: ResolutionUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    let dx = 2.0 / resolution.width;
    let dy = 2.0 / resolution.height;

    var out: VertexOutput;
    var position = vec3(
        model.position.x * resolution.cell_size * dx,
        model.position.y * resolution.cell_size * dy,
        0.0
    );
    out.clip_position = vec4<f32>(position, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}