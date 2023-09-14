#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import "shaders/light_shader.wgsl" ray_cross, Light

struct TileMaterial {
    color: vec4<f32>,
    size: vec2<f32>,
    texture_shape: vec2<f32>,
    lights: array<Light, 2>,
    light_count: u32,
};

@group(1) @binding(0)
var<uniform> material: TileMaterial;

@group(1) @binding(1)
var map_data: texture_2d<f32>;

@group(1) @binding(3)
var my_texture: texture_2d<f32>;
@group(1) @binding(4)
var my_sampler: sampler;

// 遮挡物, 可以产生阴影的物体
@group(1) @binding(5)
var obstacle: texture_2d<f32>;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    // 从 map_data 中读取当前位置的 index
    let index = u32(textureLoad(map_data, vec2<i32>(mesh.uv * material.size), 0).r);
    let tex_color: vec4<f32> = textureSample(
        my_texture, 
        my_sampler,
        // uv 对 贴图的大小取余数, 得到反复 0号 位置贴图的uv
        // 乘以 贴图大小, 得到多个块
        // 对其加上 data 中的 index * 一个uv 的偏移, 得到当前所需贴图块
        (mesh.uv % (1. / material.size)) * vec2(1.0/material.texture_shape.x, 1.0/material.texture_shape.y) * material.size +
        vec2((1./material.texture_shape.x) * f32(index % u32(material.texture_shape.x)), (1./material.texture_shape.y) * f32(index / u32(material.texture_shape.x))),
    );

    var brightness = 0.01;

    for (var i = 0u; i < material.light_count; i = i + 1u) {
        let light = material.lights[i];
        // 判断两点连线是否与四边形相交
        if ! ray_cross(obstacle, mesh.world_position.xy, light.position) {
            let d: f32 = distance(mesh.world_position.xy, light.position);
            brightness += (30.0 / d );
        }
    }

    return vec4<f32>(tex_color.rgb * brightness, 1.) * material.color;
}
