use bevy::prelude::*;

use crate::state::{AppState, ForState};

use super::{assets::UiAssets, DrawBlinkTimer};

pub fn setup(mut commands: Commands, assets: Res<UiAssets>) {
    info!("Create start menu");
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
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Glow",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                    },
                ),
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "start",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x00, 0x44, 0x44),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            ));
        });
}
