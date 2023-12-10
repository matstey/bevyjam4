use bevy::prelude::*;

use crate::despawn;
use crate::game::{GameData, GameTimer, LevelConfig};
use crate::state::{AppState, ForState};

use super::assets::UiAssets;

// This plugin manages the game screen
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::InGame), despawn::<GameScreen>)
            .add_systems(
                Update,
                (
                    update_countdown.run_if(in_state(AppState::InGame)),
                    update_present_count.run_if(in_state(AppState::InGame)),
                ),
            );
    }
}

#[derive(Component)]
struct GameScreen;

#[derive(Component)]
struct CountdownText;

#[derive(Component)]
struct PresentsText;

fn setup(mut commands: Commands, assets: Res<UiAssets>) {
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
                states: vec![AppState::InGame],
            },
            GameScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        align_self: AlignSelf::End,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    text: Text::from_section(
                        "00:00",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb_u8(0xe0, 0x1b, 0x24),
                        },
                    ),
                    ..default()
                },
                CountdownText,
            ));
            parent.spawn((
                TextBundle {
                    style: Style {
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        align_self: AlignSelf::End,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    text: Text::from_section(
                        "\n0/0", // TODO: HACK: Need to work out how to do proper UI layout
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb_u8(0xe0, 0x1b, 0x24),
                        },
                    ),
                    ..default()
                },
                PresentsText,
            ));
        });
}

fn update_countdown(timer: Res<GameTimer>, mut query: Query<&mut Text, With<CountdownText>>) {
    for mut text in query.iter_mut() {
        if let Some(text) = text.sections.first_mut() {
            // Convert secs to the min and sec components
            let total_secs = timer.remaining_secs();
            let mins = (total_secs / 60.0).floor() as i32;
            let secs = (total_secs - (mins * 60) as f32) as i32;
            text.value = format!("{:02}:{:02}", mins, secs)
        }
    }
}

fn update_present_count(
    mut query: Query<&mut Text, With<PresentsText>>,
    level_config: Res<LevelConfig>,
    game_data: Res<GameData>,
) {
    for mut text in query.iter_mut() {
        if let Some(text) = text.sections.first_mut() {
            text.value = format!(
                "\n{}/{}",
                game_data.presents_collected,
                level_config.high_orbit_presents + level_config.low_orbit_presents
            );
        }
    }
}
