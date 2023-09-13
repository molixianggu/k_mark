use crate::loader::texture::TextureAssets;
use crate::package::tile::{TileMapBundle, TileMaterial};
use crate::page::base::Page;
use crate::systems::button::button_state;
use crate::{Action, GameState};
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use leafwing_input_manager::prelude::{ActionState, InputMap, VirtualDPad};
use leafwing_input_manager::InputManagerBundle;

#[derive(Component)]
pub struct GamePage;

impl GamePage {
    fn setup(
        mut commands: Commands,
        texture: Res<TextureAssets>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut custom_materials: ResMut<Assets<CustomMaterial>>,
        mut tile_materials: ResMut<Assets<TileMaterial>>,
        mut images: ResMut<Assets<Image>>,
    ) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(64., 64.)).into())
                    .into(),
                material: custom_materials.add(CustomMaterial {
                    fill_amount: 1.0,
                    color: Color::RED.into(),
                    texture: texture.moko.clone(),
                    index: 0,
                }),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.1)),
                ..default()
            },
            InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    // (KeyCode::Space, Action::Jump),
                    // (KeyCode::ShiftLeft, Action::Run),
                    (
                        VirtualDPad {
                            up: KeyCode::Up.into(),
                            down: KeyCode::Down.into(),
                            left: KeyCode::Left.into(),
                            right: KeyCode::Right.into(),
                        },
                        Action::Move,
                    ),
                ]),
            },
            Self,
        ));
        let img = images.add(TileMapBundle::data_to_map(
            (0..30).map(|x| (x as f32) / 1000.0).collect(),
            UVec2::new(6, 5),
        ));
        commands.spawn((
            TileMapBundle::new(
                meshes
                    .add(shape::Quad::new(Vec2::new(320., 320.)).into())
                    .into(),
                tile_materials.add(TileMaterial {
                    color: Color::WHITE.into(),
                    texture_shape: Vec2::new(16., 12.),
                    size: Vec2 { x: 6., y: 5. },
                    map: [Vec4::new(0., 0., 0., 0.); 16],
                    map_data: img.clone(),
                    texture: texture.tile_a1.clone(),
                }),
                Transform::from_translation(Vec3::new(100., 100., 0.)),
            ),
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
                m.fill_amount = (time.elapsed_seconds().sin() / 2. + 0.5) * 400.0 - 200.0;
            }
        }
    }

    fn move_player(
        mut query: Query<(
            &Handle<CustomMaterial>,
            &ActionState<Action>,
            &mut Transform,
        )>,
        mut materials: ResMut<Assets<CustomMaterial>>,
        time: Res<Time>,
    ) {
        for (cm, action, mut tf) in &mut query {
            if !action.pressed(Action::Move) {
                if let Some(m) = materials.get_mut(cm) {
                    m.index = m.index / 3 * 3 + 1;
                }
                continue;
            }
            if let Some(axis) = action.axis_pair(Action::Move) {
                let mut pos = tf.translation;
                pos.x += axis.x() * 50. * time.delta_seconds();
                pos.y += axis.y() * 50. * time.delta_seconds();
                tf.translation = pos;

                let mut dir = -1;
                if axis.y() < 0. {
                    dir = 0;
                } else if axis.x() < 0. {
                    dir = 1;
                } else if axis.x() > 0. {
                    dir = 2;
                } else if axis.y() > 0. {
                    dir = 3;
                }
                if dir < 0 {
                    continue;
                }

                if let Some(m) = materials.get_mut(cm) {
                    m.index = dir as u32 * 3
                        + if (time.elapsed_seconds() * 10.).sin() > 0. {
                            0
                        } else {
                            2
                        };
                }
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
    #[uniform(0)]
    index: u32,
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
        app.add_systems(OnEnter(Self::state()), Self::setup);
        app.add_systems(
            Update,
            (Self::update_material, Self::move_player, button_state)
                .run_if(in_state(Self::state())),
        );
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
        app.add_plugins(Material2dPlugin::<TileMaterial>::default());
    }
}
