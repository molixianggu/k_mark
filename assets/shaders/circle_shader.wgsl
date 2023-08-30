#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct CustomMaterial {
    fill_amount: f32,
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@group(1) @binding(1)
var my_texture: texture_2d<f32>;

@group(1) @binding(2)
var my_sampler: sampler;


@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    var tex_color: vec4<f32> = textureSample(my_texture, my_sampler, mesh.uv * vec2(1.0, -1.0) + vec2(0.0, 1.0));
    return tex_color * material.color;
}
