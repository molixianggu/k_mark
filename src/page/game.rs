use crate::loader::texture::TextureAssets;
use crate::page::base::Page;
use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};

#[derive(Component)]
pub struct GamePage;

impl GamePage {
    fn setup(
        mut commands: Commands,
        texture: Res<TextureAssets>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut custom_materials: ResMut<Assets<CustomMaterial>>,
    ) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(300., 300.)).into())
                    .into(),
                material: custom_materials.add(CustomMaterial {
                    fill_amount: 1.0,
                    color: Color::RED.into(),
                    texture: texture.texture_bevy.clone(),
                }),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            },
            Self,
        ));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(300., 300.)).into())
                    .into(),
                material: custom_materials.add(CustomMaterial {
                    fill_amount: 1.0,
                    color: Color::RED.into(),
                    texture: texture.texture_bevy.clone(),
                }),
                transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
                ..default()
            },
            Self,
        ));
        info!("setup game page done");
    }

    fn update_material(
        query: Query<&Handle<CustomMaterial>>,
        time: Res<Time>,
        mut materials: ResMut<Assets<CustomMaterial>>,
    ) {
        for cm in &query {
            if let Some(m) = materials.get_mut(cm) {
                m.fill_amount = ((time.elapsed_seconds() as f32).sin() / 2. + 0.5) * 400.0 - 200.0;
            }
        }
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

    fn name() -> &'static str {
        "game"
    }
    fn state() -> GameState {
        GameState::Game
    }

    fn build(app: &mut App) {
        app.add_systems(OnEnter(Self::state()), (Self::setup,));
        app.add_systems(Update, (Self::update_material,));
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
    }
}
