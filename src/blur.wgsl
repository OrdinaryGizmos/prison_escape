 
struct VertexOutput{
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] tex_coord: vec2<f32>;
};

[[block]]
struct Uniforms{
    camera_transform: mat4x4<f32>;
    camera_inverse_transform: mat4x4<f32>;
    camera_position: vec3<f32>;
    screen_width: f32;
    screen_height: f32;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

[[group(0), binding(1)]]
var r_texture: texture_2d<f32>;
[[group(0), binding(2)]]
var r_sampler: sampler;


[[stage(vertex)]]
fn vs_main(
           [[location(0)]] in_position: vec3<f32>,
           [[location(1)]] in_tex_coord: vec2<f32>,
           ) -> VertexOutput
{
    var v_out: VertexOutput;
    v_out.pos = vec4<f32>(in_position, 1.0);
    v_out.tex_coord = in_tex_coord;
    return v_out;
}
[[stage(fragment)]]
fn fs_main( in: VertexOutput) -> [[location(0)]] vec4<f32>{
    var sum: vec3<f32> = vec3<f32>(0.0);
    let offset = vec2<f32>(1.0 / uniforms.screen_width, 1.0 / uniforms.screen_height);
    for(var x: f32 = -2.0; x < 3.0; x = x + 1.0) {
        for(var y: f32 = -2.0; y < 3.0; y = y + 1.0) {
            let coord = vec2<f32>(in.tex_coord + (offset * vec2<f32>(x, y)));
            sum = sum + textureSample(r_texture, r_sampler, coord).xyz;
        }
    }
    sum = sum / 25.0;
    return vec4<f32>(sum, 1.0);
}
