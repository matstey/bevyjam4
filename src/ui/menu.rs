use bevy::{app::AppExit, prelude::*};

use crate::asset::LoadingAssets;
use crate::despawn;
use crate::state::{AppState, ForState};

use super::assets::UiAssets;

// This plugin manages the start menu
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(AppState::StartMenu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn::<OnMainMenuScreen>)
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(AppState::StartMenu)),
            );
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;

const NORMAL_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.0);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

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
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
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
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Santa F**cked Up",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
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
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    commands.insert_resource(LoadingAssets::default()); // TODO: Just a little hack for testing
                    game_state.set(AppState::GameLoading);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}
