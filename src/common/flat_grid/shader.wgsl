struct ResolutionUniform {
    width: f32,
    height: f32,
    cell_size: f32, // minimum binding size should be 16 bytes
    _offset2: f32,
}

@group(0) @binding(0)
var <uniform> resolution: ResolutionUniform;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, 1.0)
    );

    var out: VertexOutput;
    out.position = vec4<f32>(pos[index], 0.0, 1.0);

    return out;
}

fn get_uvs(coord: vec2<f32>) -> vec2<f32> {
    var uv = coord / vec2<f32>(resolution.width, resolution.height);

    uv.y = 1.0 - uv.y;
    return uv;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let cell_color = vec4f(1.0, 1.0, 1.0, 1.0);
    let border_color = vec4f(0.4235, 0.4784, 0.5372, 1.0);
    let cell_size = i32(resolution.cell_size);

    let resolution_x = i32(resolution.width);
    let amount_cells_by_x = (resolution_x / cell_size);
    let gap_x = ((resolution_x % cell_size) / 2) + select(0, cell_size / 2, (amount_cells_by_x % 2) == 1);

    let resolution_y = i32(resolution.height) ;
    let amount_cells_by_y = (resolution_y / cell_size);
    let gap_y = ((resolution_y % cell_size) / 2) + select(0, cell_size / 2, (amount_cells_by_y % 2) == 1);

    let grid_x = i32(in.position.x);
    let grid_y = i32(in.position.y);
    let checker = ((grid_x % cell_size) == gap_x) || ((grid_y % cell_size) == gap_y);

    return select(cell_color, border_color, checker);
}