use bevy::{
    diagnostic::{
        DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};

use super::assets::UiAssets;

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update)
            .add_plugins((
                FrameTimeDiagnosticsPlugin,
                SystemInformationDiagnosticsPlugin,
            ));
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct CpuText;

pub fn setup(mut commands: Commands, assets: Res<UiAssets>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: assets.font.clone(),
                font_size: 16.0,
                color: Color::GOLD,
            }),
            #[cfg(not(target_arch = "wasm32"))]
            TextSection::new(
                "\nCPU: ",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            #[cfg(not(target_arch = "wasm32"))]
            TextSection::from_style(TextStyle {
                font: assets.font.clone(),
                font_size: 16.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        FpsText,
    ));
}

pub fn update(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.0}");
            }
        }

        if let Some(cpu) = diagnostics.get(SystemInformationDiagnosticsPlugin::CPU_USAGE) {
            if let Some(value) = cpu.smoothed() {
                text.sections[3].value = format!("{value:.0}");
            }
        }
    }
}
