use bevy::prelude::*;

use crate::system_sets::StartupSystems;

#[derive(Resource, Default)]
pub struct Models {
    pub house: Handle<Scene>,
}

fn load_building_assets(
    server: Res<AssetServer>, 
    mut models: ResMut<Models>    
) {
    models.house = server.load(GltfAssetLabel::Scene(0).from_asset("kenney_hexagon-kit/Models/GLB format/unit-house.glb"))
}


/* Init Plugin */
pub struct GameModelsPlugin;

impl Plugin for GameModelsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Models>()
            .add_systems(Startup, load_building_assets.in_set(StartupSystems::LoadAssets));
    }
}