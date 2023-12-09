use bevy::prelude::*;

use crate::state::AppState;

pub mod assets;
pub mod colors;
pub mod diagnostics;
pub mod end;
pub mod game;
pub mod loading;
pub mod paused;
pub mod splash;
pub mod start;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, assets::setup)
            .add_systems(Update, button_system);
    }
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                colors::PRESSED_BUTTON.into()
            }
            (Interaction::Hovered, Some(_)) => colors::HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => colors::HOVERED_BUTTON.into(),
            (Interaction::None, None) => colors::NORMAL_BUTTON.into(),
        }
    }
}
