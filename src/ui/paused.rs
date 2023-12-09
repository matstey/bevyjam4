use bevy::prelude::*;

use crate::despawn;
use crate::state::{AppState, ForState, GameState};

use super::assets::UiAssets;

// This plugin manages the start menu
pub struct PausedMenuPlugin;

impl Plugin for PausedMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), menu_setup)
            .add_systems(OnExit(GameState::Paused), despawn::<PausedMenuScreen>)
            .add_systems(Update, menu_action.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct PausedMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Resume,
    Quit,
}

fn menu_setup(mut commands: Commands, assets: Res<UiAssets>) {
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
            PausedMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Paused",
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
                    MenuButtonAction::Resume,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "resume",
                            TextStyle {
                                font: assets.font.clone(),
                                font_size: 50.0,
                                color: Color::rgb_u8(0xe0, 0x1b, 0x24),
                            },
                        ),
                        ..default()
                    },));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style { ..default() },
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "quit",
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
                MenuButtonAction::Quit => {
                    app_state.set(AppState::StartMenu);
                    game_state.set(GameState::Running);
                }
                MenuButtonAction::Resume => {
                    game_state.set(GameState::Running);
                }
            }
        }
    }
}
