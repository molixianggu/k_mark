#![allow(clippy::type_complexity)]

mod page;
mod loader;

mod systems;

use bevy::app::App;
use bevy::prelude::*;


#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,

    Title,
}

impl GameState {
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            loader::AssetLoadPlugin,
            page::PagePlugin::new(),
        ));

        #[cfg(debug_assertions)]
        {
            // app.add_plugins((bevy::diagnostic::FrameTimeDiagnosticsPlugin, bevy::diagnostic::LogDiagnosticsPlugin::default()));
        }
    }
}
