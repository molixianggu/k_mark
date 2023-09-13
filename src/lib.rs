#![allow(clippy::type_complexity)]

mod loader;
mod package;
mod page;

mod systems;

use bevy::app::App;
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;
use leafwing_input_manager::Actionlike;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,

    Title,
    Game,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Run,
    Jump,
    // 移动
    Move,
}

impl GameState {}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_plugins((loader::AssetLoadPlugin, page::PagePlugin::new()));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                // bevy::diagnostic::FrameTimeDiagnosticsPlugin,
                // bevy::diagnostic::LogDiagnosticsPlugin::default(),
                // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            ));
        }
    }
}
