use bevy::prelude::*;
use crate::GameState;

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
            .add_systems(OnExit(Self::state()), (
                Self::SelfType::teardown,
            ));
    }
}

