use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,

    #[asset(path = "textures/button_0.png")]
    pub button_background: Handle<Image>,
    #[asset(path = "textures/button_1.png")]
    pub button_background_up: Handle<Image>,
    // moko
    #[asset(path = "textures/moko.png")]
    pub moko: Handle<Image>,
    // moko
    #[asset(path = "textures/TileA1.png")]
    pub tile_a1: Handle<Image>,
}
