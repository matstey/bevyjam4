use bevy::{app::AppExit, prelude::*};

use crate::asset::LoadingAssets;
use crate::despawn;
use crate::state::{AppState, ForState};

use super::assets::UiAssets;

// This plugin manages the start menu
pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StartMenuState>()
            .add_systems(OnEnter(AppState::StartMenu), menu_setup)
            .add_systems(OnEnter(StartMenuState::Main), main_menu_setup)
            .add_systems(OnExit(StartMenuState::Main), despawn::<StartMenuScreen>)
            .add_systems(Update, menu_action.run_if(in_state(AppState::StartMenu)));
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum StartMenuState {
    Main,
    #[default]
    Disabled,
}

#[derive(Component)]
struct StartMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    #[cfg(not(target_arch = "wasm32"))]
    Quit,
}

fn menu_setup(mut menu_state: ResMut<NextState<StartMenuState>>) {
    menu_state.set(StartMenuState::Main);
}

fn main_menu_setup(mut commands: Commands, assets: Res<UiAssets>) {
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
            StartMenuScreen,
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
            parent
                .spawn((
                    ButtonBundle {
                        style: Style { ..default() },
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        ..default()
                    },
                    MenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "lets go!",
                            TextStyle {
                                font: assets.font.clone(),
                                font_size: 50.0,
                                color: Color::rgb_u8(0xe0, 0x1b, 0x24),
                            },
                        ),
                        ..default()
                    },));
                });
            #[cfg(not(target_arch = "wasm32"))] // Cannot quit on wasm
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
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<StartMenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                #[cfg(not(target_arch = "wasm32"))]
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    commands.insert_resource(LoadingAssets::default()); // TODO: Just a little hack for testing
                    app_state.set(AppState::Loading);
                    menu_state.set(StartMenuState::Disabled);
                }
            }
        }
    }
}
