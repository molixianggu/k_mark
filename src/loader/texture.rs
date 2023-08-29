use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,

    #[asset(path = "textures/9.png")]
    pub button_background: Handle<Image>,
    #[asset(path = "textures/9_1.png")]
    pub button_background_up: Handle<Image>,
}