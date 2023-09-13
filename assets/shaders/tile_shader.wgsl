#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct TileMaterial {
    color: vec4<f32>,
    size: vec2<f32>,
    texture_shape: vec2<f32>,
    map: array<vec4<u32>, 16>,
};

@group(1) @binding(0)
var<uniform> material: TileMaterial;

@group(1) @binding(1)
var map_data: texture_2d<f32>;

@group(1) @binding(3)
var my_texture: texture_2d<f32>;
@group(1) @binding(4)
var my_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let p = textureLoad(map_data, vec2<i32>(mesh.uv * material.size), 0);
    let index = u32(p.r * 1000.0);
    let tex_color: vec4<f32> = textureSample(
        my_texture, 
        my_sampler, 
        (mesh.uv % (1. / material.size)) * vec2(1.0/material.texture_shape.x, 1.0/material.texture_shape.y) * material.size.x + 
        vec2((1./material.texture_shape.x) * f32(index % u32(material.texture_shape.x)), (1./material.texture_shape.y) * f32(index / u32(material.texture_shape.x))),
    );
    return tex_color * material.color;
}
