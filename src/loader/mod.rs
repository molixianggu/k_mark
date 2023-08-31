use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod font;
pub mod texture;

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), (setup,))
            .add_systems(OnExit(GameState::Loading), (done,))
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Title),
            )
            .add_collection_to_loading_state::<_, font::FontAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, texture::TextureAssets>(GameState::Loading);
    }
}

fn setup(mut _commands: Commands) {
    info!("loading setup");
}

fn done(mut _commands: Commands) {
    info!("loading done");
}
