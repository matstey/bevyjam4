use bevy::prelude::*;

use crate::despawn;
use crate::state::{AppState, ForState};

use super::assets::UiAssets;

// This plugin manages the loading screen
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameLoading), setup)
            .add_systems(OnExit(AppState::GameLoading), despawn::<OnLoadingScreen>);
    }
}

#[derive(Component)]
struct OnLoadingScreen;

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
                states: vec![AppState::GameLoading],
            },
            OnLoadingScreen,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Loading..",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: Color::rgb_u8(0xe0, 0x1b, 0x24),
                    },
                ),
                ..default()
            },));
        });
}
