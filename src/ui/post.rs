use bevy::prelude::*;

use crate::despawn;
use crate::game::GameData;
use crate::state::{AppState, ForState, GameState};

use super::assets::UiAssets;

// This plugin manages the post game ui
pub struct PostGamePlugin;

impl Plugin for PostGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::PostGame), setup)
            .add_systems(OnExit(AppState::PostGame), despawn::<PostGameScreen>)
            .add_systems(Update, menu_action.run_if(in_state(AppState::PostGame)));
    }
}

#[derive(Component)]
struct PostGameScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Continue,
}

fn setup(mut commands: Commands, assets: Res<UiAssets>, game_data: Res<GameData>) {
    let title = if game_data.won {
        "You Won!"
    } else {
        "Game Over"
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![AppState::StartMenu],
            },
            PostGameScreen,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    title,
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 80.0,
                        color: Color::rgb_u8(0xe0, 0x1b, 0x24),
                    },
                ),
                ..default()
            },));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style { ..default() },
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        ..default()
                    },
                    MenuButtonAction::Continue,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "continue",
                            TextStyle {
                                font: assets.font.clone(),
                                font_size: 50.0,
                                color: Color::rgb_u8(0xe0, 0x1b, 0x24),
                            },
                        ),
                        ..default()
                    },));
                });
        });
}

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Continue => {
                    app_state.set(AppState::StartMenu);
                    game_state.set(GameState::Running);
                }
            }
        }
    }
}
