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

// 判断两点连线是否与四边形相交
// fn ray_cross(shape: array<vec2<f32>, 4>, light_point: vec2, target_point: vec2) -> f32 {
//     for i in range(0, 4) {
//         let p1: vec2<f32> = shape[i];
//         let p2: vec2<f32> = shape[(i + 1) % 4];

//         let v1: vec2<f32> = p2 - p1;
//         let v2: vec2<f32> = light_point - target_point;

//         let cross1: f32 = cross(v1, v2);
//         let cross2: f32 = cross(v2, v3);

//         if cross1 * cross2 < 0.0 {
//             return 0.0;
//         }
//     }
//     return 1.0;
// }

// fn crossx(v1: vec2<f32>, v2: vec2<f32>) -> f32 {
//     return v1.x * v2.y - v1.y * v2.x;
// }

const EPSINON = 1e-5;

fn is_intersect(a: vec2<f32>, b: vec2<f32>, c: vec2<f32>, d: vec2<f32>) -> bool {
    let area_abc = (a.x - c.x) * (b.y - c.y) - (a.y - c.y) * (b.x - c.x);
    let area_abd = (a.x - d.x) * (b.y - d.y) - (a.y - d.y) * (b.x - d.x);

    if ( area_abc * area_abd >= - EPSINON ) {
        return false;
    }

    let area_cda = (c.x - a.x) * (d.y - a.y) - (c.y - a.y) * (d.x - a.x);
    let area_cdb = area_cda + area_abc - area_abd ;
    
    if ( area_cda * area_cdb >= - EPSINON ) {
        return false;
    }

    return true;
}


fn ray_cross(shape: array<vec2<f32>, 4>, light_point: vec2<f32>, target_point: vec2<f32>) -> bool {
    return 
        is_intersect(light_point, target_point, shape[0], shape[1]) ||
        is_intersect(light_point, target_point, shape[1], shape[2]) || 
        is_intersect(light_point, target_point, shape[2], shape[3]) ||
        is_intersect(light_point, target_point, shape[3], shape[0])
    ;
}

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let shape: array<vec2<f32>, 4> = array<vec2<f32>, 4>(
        vec2<f32>(-30.0, -30.0),
        vec2<f32>(30.0, -30.0),
        vec2<f32>(30.0, 30.0),
        vec2<f32>(-30.0, 30.0),
    );

    let target_point: vec2<f32> = vec2<f32>(material.fill_amount, 100.0);
    let light_point: vec2<f32> = mesh.world_position.xy;

    let tex_color: vec4<f32> = textureSample(my_texture, my_sampler, mesh.uv * vec2(1.0, -1.0) + vec2(0.0, 1.0));

    var pow = 0.0;

    // 判断两点连线是否与四边形相交
    if ! ray_cross(shape, light_point, target_point) {
        pow += 30.0;
    }


    let d: f32 = distance(light_point, target_point);
    let brightness: f32 = (pow / d ) + 0.05;

    
    return vec4<f32>((tex_color * brightness * material.color).xyz, 1.0);
}
