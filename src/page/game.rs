use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use crate::GameState;
use crate::page::base::Page;
use crate::loader::texture::TextureAssets;

#[derive(Component)]
pub struct GamePage;


impl GamePage {
    fn setup(mut commands: Commands, texture: Res<TextureAssets>, mut meshes: ResMut<Assets<Mesh>>, mut custom_materials: ResMut<Assets<CustomMaterial>>) {
        commands.spawn(
            (
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::new(Vec2::new(100., 100.)).into()).into(),
                    material: custom_materials.add(CustomMaterial {
                        fill_amount: 1.0,
                        color: Color::WHITE.into(),
                        texture: texture.texture_bevy.clone(),
                    }),
                    ..default()
                },
                Self,
            )
        );
        info!("setup game page done");
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "5b5569c8-36d4-4c9d-acb7-d1754b385ab2"]
struct CustomMaterial {
    #[uniform(0)]
    fill_amount: f32,
    #[uniform(0)]
    color: Vec4,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle_shader.wgsl".into()
    }
}

impl Page for GamePage {
    type SelfType = Self;

    fn name() -> &'static str { "game" }
    fn state() -> GameState {
        GameState::Game
    }

    fn build(app: &mut App) {
        app.add_systems(OnEnter(Self::state()), (
            Self::setup,
        ));
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
    }
}

