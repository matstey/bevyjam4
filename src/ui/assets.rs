use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    //pub ship_life: UiImage,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        //ship_life: asset_server.load("playerLife1_red.png").into(),
    });
}
