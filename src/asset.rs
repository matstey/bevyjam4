use bevy::{asset::UntypedAssetId, ecs::system::Resource};

#[derive(Resource, Default)]
pub struct LoadingAssets(pub Vec<UntypedAssetId>);
