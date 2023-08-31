use crate::loader::texture::TextureAssets;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

#[derive(WorldQuery)]
pub struct ButtonQueryFilter<T: Component> {
    _button: With<Button>,
    _change: Changed<Interaction>,
    _t: With<T>,
}

pub fn on_click<T: Component>(query: Query<&Interaction, ButtonQueryFilter<T>>) -> bool {
    for interaction in query.iter() {
        match *interaction {
            Interaction::Pressed => {
                return true;
            }
            _ => {}
        }
    }
    false
}

pub fn button_state(
    mut query: Query<(&Interaction, &mut UiImage, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    texture: Res<TextureAssets>,
) {
    for (interaction, mut image, children) in query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                image.texture = texture.button_background.clone();
                text.sections[0].style.color = Color::BLACK;
            }
            Interaction::Hovered => {
                image.texture = texture.button_background_up.clone();
                text.sections[0].style.color = Color::GRAY;
            }
            Interaction::None => {
                image.texture = texture.button_background.clone();
                text.sections[0].style.color = Color::DARK_GRAY;
            }
        }
    }
}
