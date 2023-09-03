use crate::GameState;
use bevy::prelude::*;

pub trait Page {
    type SelfType: 'static + Component + Page;

    fn name() -> &'static str;
    fn state() -> GameState;

    fn build(app: &mut App);

    fn teardown(mut commands: Commands, query: Query<Entity, With<Self::SelfType>>) {
        for entity in &mut query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn register(app: &mut App) {
        Self::build(app);
        app
            // 离开页面时，执行 teardown 方法
            .add_systems(OnExit(Self::state()), (Self::SelfType::teardown,));
    }

    fn crate_button<T: Component + Default>(
        parent: &mut ChildBuilder,
        text: &str,
        texture: Handle<Image>,
        font: Handle<Font>,
    ) {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(100.0),
                        ..Default::default()
                    },
                    image: UiImage::new(texture.clone()),
                    ..Default::default()
                },
                T::default(),
            ))
            .with_children(|p| {
                p.spawn(TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::DARK_GRAY,
                        },
                    ),
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                });
            });
    }
}
