use crate::loader::font::FontAssets;
use crate::loader::texture::TextureAssets;
use crate::page::base::Page;
use crate::systems::button::{button_state, on_click};
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct TitlePage;

impl TitlePage {
    fn setup(mut commands: Commands, texture: Res<TextureAssets>, font: Res<FontAssets>) {
        info!("setup title page");
        commands
            .spawn((NodeBundle { ..default() }, Self))
            .with_children(|parent| {
                Self::crate_button::<StartGameButton>(
                    parent,
                    "Start Game",
                    texture.button_background.clone(),
                    font.fira_sans.clone(),
                );
                Self::crate_button::<SettingsButton>(
                    parent,
                    "Settings",
                    texture.button_background.clone(),
                    font.fira_sans.clone(),
                );
            });
    }

    fn start(mut state: ResMut<NextState<GameState>>) {
        info!("start game !");
        state.set(GameState::Game);
    }

    fn settings() {
        info!("settings !")
    }
}

#[derive(Component, Default)]
struct StartGameButton;

#[derive(Component, Default)]
struct SettingsButton;

impl Page for TitlePage {
    type SelfType = Self;

    fn name() -> &'static str {
        "title"
    }
    fn state() -> GameState {
        GameState::Title
    }

    fn build(app: &mut App) {
        app.add_systems(OnEnter(Self::state()), (Self::setup,));

        app.add_systems(
            Update,
            (
                Self::start.run_if(on_click::<StartGameButton>),
                Self::settings.run_if(on_click::<SettingsButton>),
                button_state,
            )
                .run_if(in_state(Self::state())),
        );
    }
}
