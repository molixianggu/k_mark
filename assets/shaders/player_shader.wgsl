#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import "shaders/light_shader.wgsl" ray_cross, Light

struct CustomMaterial {
    lights: array<Light, 2>,
    light_count: u32,
    color: vec4<f32>,
    index: u32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@group(1) @binding(1)
var my_texture: texture_2d<f32>;

@group(1) @binding(2)
var my_sampler: sampler;

// 遮挡物, 可以产生阴影的物体
@group(1) @binding(3)
var obstacle: texture_2d<f32>;


@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let tex_color: vec4<f32> = textureSample(
        my_texture, 
        my_sampler, 
        mesh.uv * vec2(1.0/3.0, 1.0/12.0) + vec2(f32(material.index % 3u) * (1.0 / 3.0), f32(material.index / 3u) * (1.0 / 12.0))
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
    return vec4<f32>((tex_color * brightness).xyz, tex_color.a) * material.color;
}
