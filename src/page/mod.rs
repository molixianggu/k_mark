use crate::page::base::Page;
use bevy::prelude::*;

mod base;
mod game;
mod title;

pub struct PagePlugin {}

impl PagePlugin {
    pub fn new() -> Self {
        Self {}
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        title::TitlePage::register(app);
        game::GamePage::register(app);
    }
}
