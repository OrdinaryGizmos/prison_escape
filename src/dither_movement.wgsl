//INPUTS: Camera, Camera Velocity, RenderPass 1 texture, RenderPass 2 texture
struct VertexInput {
    [[location(0)]] v_position: vec3<f32>;
    [[location(1)]] v_tex_coords: vec3<f32>;
    [[location(2)]] v_normal: vec3<f32>;
    [[location(3)]] v_color: vec4<f32>;
};

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
var r_sampler: sampler;

[[group(1), binding(0)]]
var r_texture: texture_2d<f32>;


[[stage(vertex)]]
fn vs_main(
           in_vertex: VertexInput
           ) -> VertexOutput
{
    var v_out: VertexOutput;
    v_out.pos = vec4<f32>(in_vertex.v_position, 1.0);
    v_out.tex_coord = in_vertex.v_tex_coords.xy;
    return v_out;
}


fn random(coords: vec2<f32>) -> f32 {
    return fract(sin(dot(coords.xy, vec2<f32>(12.9898,78.233))) * 43758.5453);
}

fn sample_offset(tex_coords: vec2<f32>, offset: vec2<f32>) -> vec4<f32>{
    let x_off = 1.0 / uniforms.screen_width;
    let y_off = 1.0 / uniforms.screen_height;
    return textureSample(r_texture,
                  r_sampler,
                  tex_coords + vec2<f32>(offset.x * x_off, offset.y * y_off));
}

fn sobel(coords: vec2<f32>) -> vec4<f32>{
    var x: vec4<f32> = vec4<f32>(0.0);
    var y: vec4<f32> = vec4<f32>(0.0);
    x = x + sample_offset(coords, vec2<f32>(-1.0, -1.0)) * -1.0;
    x = x + sample_offset(coords, vec2<f32>(-1.0,  0.0)) * -2.0;
    x = x + sample_offset(coords, vec2<f32>(-1.0,  1.0)) * -1.0;

    x = x + sample_offset(coords, vec2<f32>( 1.0, -1.0)) *  1.0;
    x = x + sample_offset(coords, vec2<f32>( 1.0,  0.0)) *  2.0;
    x = x + sample_offset(coords, vec2<f32>( 1.0,  1.0)) *  1.0;

    y = y + sample_offset(coords, vec2<f32>(-1.0, -1.0)) * -1.0;
    y = y + sample_offset(coords, vec2<f32>( 0.0, -1.0)) * -2.0;
    y = y + sample_offset(coords, vec2<f32>( 1.0, -1.0)) * -1.0;

    y = y + sample_offset(coords, vec2<f32>(-1.0,  1.0)) *  1.0;
    y = y + sample_offset(coords, vec2<f32>( 0.0, -1.0)) *  2.0;
    y = y + sample_offset(coords, vec2<f32>( 1.0,  1.0)) *  1.0;

    return sqrt(x * x + y * y);
}

[[stage(fragment)]]
fn fs_main( in: VertexOutput) -> [[location(0)]] vec4<f32>{
    var output_color: vec4<f32> = vec4<f32>(0.0);
    let x_coord = (in.pos.x - (f32(uniforms.screen_width)  / 2.0)) / f32(uniforms.screen_width);
    let y_coord = (in.pos.y - (f32(uniforms.screen_height) / 2.0)) / f32(uniforms.screen_height);
    let x_off = 1.0 / uniforms.screen_width;
    let y_off = 1.0 / uniforms.screen_height;
    let distance_from_center = sqrt((x_coord * x_coord) + (y_coord * y_coord));
    var sum: vec4<f32> = sobel(in.tex_coord.xy);
    // for(var x: i32 = -2; x < 3; x = x + 1){
    //     for(var y: i32 = -2; y < 3; y = y + 1){
    //         sum = sum + sample_offset(in.tex_coord.xy, vec2<f32>(f32(x), f32(y)));//(sobel(in.tex_coord + vec2<f32>(f32(x) * x_off, f32(y) * y_off)) * vec4<f32>(1.0, 0.0, 0.0, 1.0));
    //     }
    // }
    // output_color = sum / 25.0;
    //let NOISE: f32 = 15.0 / 255.0;
    // let alpha_blend: f32 = 1.0;//((distance_from_center * distance_from_center) * 2.0) + mix(-NOISE, NOISE, random(vec2<f32>(x_coord, y_coord)));

    // return vec4<f32>(output_color.xyz, output_color.w * alpha_blend);
    return textureSample(r_texture,
                  r_sampler,
                         in.tex_coord.xy);
}

