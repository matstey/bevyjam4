use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub background: UiImage,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        background: asset_server.load("thumb.png").into(),
    });
}
