use bevy::prelude::*;

use super::{assets::UiAssets, AppState};
use crate::despawn;

// This plugin will display a splash screen with a logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
            .add_systems(OnExit(AppState::Splash), despawn::<OnSplashScreen>);
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    //width: Val::Px(200.0),
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    ..default()
                },
                image: assets.background.clone(),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        app_state.set(AppState::StartMenu);
    }
}
