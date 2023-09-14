use bevy::render::render_resource::ShaderType;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat},
    sprite::{Material2d, Mesh2dHandle},
};

#[derive(AsBindGroup, ShaderType, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f18e2a0892674f6c8ab496289805c08c"]
pub struct Light {
    #[uniform(0)]
    pub position: Vec2,
    #[uniform(0)]
    pub color: Vec3,
    #[uniform(0)]
    pub radius: f32,
    #[uniform(0)]
    pub intensity: f32,
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "d21b510e-6a52-4147-b45a-f47f5df006e1"]
pub struct TileMaterial {
    #[uniform(0)]
    pub color: Vec4,
    #[uniform(0)]
    pub size: Vec2,
    #[uniform(0)]
    pub texture_shape: Vec2,
    #[uniform(0)]
    pub lights: [Light; 2],
    #[uniform(0)]
    pub light_count: u32,
    #[texture(1)]
    pub map_data: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub texture: Handle<Image>,
    #[texture(5)]
    pub obstacle: Handle<Image>,
}

impl Material2d for TileMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tile_shader.wgsl".into()
    }
}

#[derive(Bundle, Clone)]
pub struct TileMapBundle<M: Material2d> {
    pub mesh: Mesh2dHandle,
    pub material: Handle<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl TileMapBundle<TileMaterial> {
    pub fn new(mesh: Mesh2dHandle, material: Handle<TileMaterial>, transform: Transform) -> Self {
        Self {
            mesh,
            material,
            transform,
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }

    pub fn data_to_map(data: Vec<f32>, size: UVec2) -> Image {
        let mut bits: Vec<u8> = vec![];
        for row in data {
            bits.extend_from_slice(&row.to_ne_bytes());
        }
        Image::new(
            Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            bits,
            TextureFormat::R32Float,
        )
    }

    pub fn data_to_1d_texture(data: Vec<Vec2>, size: u32) -> Image {
        let mut bits: Vec<u8> = vec![];
        for row in data {
            bits.extend_from_slice(&row.x.to_ne_bytes());
            bits.extend_from_slice(&row.y.to_ne_bytes());
        }
        Image::new(
            Extent3d {
                width: size,
                height: 1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            bits,
            TextureFormat::Rg32Float,
        )
    }
}
