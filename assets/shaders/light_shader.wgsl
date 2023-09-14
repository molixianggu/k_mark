
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

struct Light {
    position: vec2<f32>,
    color: vec3<f32>,
    radius: f32,
    intensity: f32,
}

// 判断两点连线是否与图形相交
fn ray_cross(obstacle: texture_2d<f32>, light_point: vec2<f32>, target_point: vec2<f32>) -> bool {
    let layer = 1;
//    let layer = textureNumLayers(obstacle);
    let len = i32(textureDimensions(obstacle).x);
    for (var l: i32 = 0; l<layer; l++) {
        for (var i: i32 ; i<(len - 1); i++) {
            let v = textureLoad(obstacle, vec2<i32>(i, 0), l).rg;
            let u = textureLoad(obstacle, vec2<i32>(i + 1, 0), l).rg;
            if (is_intersect(light_point, target_point, v, u)) {
                return true;
            }
        }
    }
    return false;
}
