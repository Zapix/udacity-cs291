struct ResolutionUniform {
    width: f32,
    height: f32,
    cell_size: f32,
    _offset: f32,
}

@group(0) @binding(0)
var <uniform> resolution: ResolutionUniform;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var dx = 2.0 / resolution.width; // fraction of 1px by x
    var dy = 2.0 / resolution.height; // fraction of 1px by y

    var one_cell_x = dx * resolution.cell_size;
    var one_cell_y = dy * resolution.cell_size;

    var padding_in_cells = 2.0;
    var left_x = -(1.0 - padding_in_cells * one_cell_x);
    var right_x = (1.0 - padding_in_cells * one_cell_x);
    var top_y = (1.0 - padding_in_cells * one_cell_y);
    var bottom_y = -(1.0 - padding_in_cells * one_cell_y);

    var width = 4.0; // width of line in pixels
    var width_x = width * dx;
    var width_y = width * dy;

    var arrow_width = 0.15 * resolution.cell_size; // in pixels
    var arrow_length = 0.5 * resolution.cell_size; // in pixels
    var arrow_width_x = arrow_width * dx;
    var arrow_length_x = arrow_length * dx;
    var arrow_width_y = arrow_width * dy;
    var arrow_length_y = arrow_length * dy;

    var pos = array<vec2<f32>, 18>(
        vec2<f32>( left_x, -(width_y / 2.0)),
        vec2<f32>( right_x, -(width_y / 2.0)),
        vec2<f32>( left_x, width_y / 2.0),

        vec2<f32>( right_x, -(width_y / 2.0)),
        vec2<f32>( right_x, width_y / 2.0),
        vec2<f32>( left_x, width_y / 2.0),

        vec2<f32>( right_x, -arrow_width_y),
        vec2<f32>( right_x + arrow_length_x, 0.0),
        vec2<f32>( right_x, arrow_width_y),

        vec2<f32>(-(width_x/2.0), top_y),
        vec2<f32>(-(width_x/2.0), bottom_y),
        vec2<f32>(width_x/2.0, bottom_y),

        vec2<f32>(width_x/2.0, bottom_y),
        vec2<f32>(width_x/2.0, top_y),
        vec2<f32>(-(width_x/2.0), top_y),

        vec2<f32>(0.0, top_y + arrow_length_y),
        vec2<f32>(-arrow_width_x, top_y),
        vec2<f32>(arrow_width_x, top_y),
    );

    var out: VertexOutput;
    out.position = vec4(pos[vertex_index], 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(0.0, 0.0, 0.0, 1.0);
}
