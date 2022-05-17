//https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct MyMat {
    alpha: f32;
    color: vec4<f32>;
};

[[group(1), binding(0)]]
var<uniform> uniform_data: MyMat;

[[group(1), binding(1)]]
var texture: texture_2d<f32>;
[[group(1), binding(2)]]
var our_sampler: sampler;

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var output_color = vec4<f32>(input.uv,0.0,uniform_data.alpha);
    output_color = output_color * textureSample(texture, our_sampler, input.uv);
    output_color = output_color * uniform_data.color;
    return output_color;
}


