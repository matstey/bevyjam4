use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    prelude::*,
};

use crate::state::AppState;

pub mod assets;
pub mod diagnostics;
pub mod start_menu;

pub struct UiPlugin {
    show_diag: bool,
}

impl UiPlugin {
    pub fn new(show_diag: bool) -> Self {
        Self { show_diag }
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(OnEnter(AppState::GamePaused), pause_menu)
            //.add_systems(OnEnter(AppState::GameOver), gameover_menu)
            //.add_systems(Update, (menu_input_system, menu_blink_system))
            .add_systems(Startup, assets::setup);

        if self.show_diag {
            app.add_systems(OnEnter(AppState::StartMenu), diagnostics::setup)
                .add_systems(Update, diagnostics::update)
                .add_plugins((
                    FrameTimeDiagnosticsPlugin,
                    SystemInformationDiagnosticsPlugin,
                ));
        }
    }
}

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);
