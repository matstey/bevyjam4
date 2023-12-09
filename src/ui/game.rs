use bevy::prelude::*;

use crate::despawn;
use crate::game::GameTimer;
use crate::state::{AppState, ForState};

use super::assets::UiAssets;

// This plugin manages the game screen
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::InGame), despawn::<GameScreen>)
            .add_systems(Update, update.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct GameScreen;

#[derive(Component)]
struct CountdownText;

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
        });
}

fn update(timer: Res<GameTimer>, mut query: Query<&mut Text, With<CountdownText>>) {
    for mut text in query.iter_mut() {
        match text.sections.first_mut() {
            Some(text) => {
                text.value = format!(
                    "{:00}:{:00}",
                    timer.remaining().as_secs(),
                    timer.remaining().subsec_millis()
                )
            }
            None => {}
        }
    }
}
