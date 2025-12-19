use bevy::prelude::*;

use crate::system_sets::StartupSystems;

#[derive(Resource, Default)]
pub struct Models {
    pub house: Handle<Scene>,
    pub farm: Handle<Scene>,
    pub lumber_mill: Handle<Scene>,
    pub stone_mine: Handle<Scene>,
    pub gold_mine: Handle<Scene>,
    pub castle: Handle<Scene>,
}

fn load_building_assets(
    server: Res<AssetServer>, 
    mut models: ResMut<Models>    
) {
    models.house = server.load(GltfAssetLabel::Scene(0).from_asset("kenney_hexagon-kit/Models/GLB format/unit-house.glb"));
    models.farm = server.load(GltfAssetLabel::Scene(0).from_asset("kenney_hexagon-kit/Models/GLB format/unit-mill.glb"));
    models.lumber_mill = server.load(GltfAssetLabel::Scene(0).from_asset("kenney_hexagon-kit/Models/GLB format/unit-house.glb"));
    models.stone_mine = server.load(GltfAssetLabel::Scene(0).from_asset("kenney_hexagon-kit/Models/GLB format/unit-house.glb"));
    models.gold_mine = server.load(GltfAssetLabel::Scene(0).from_asset("kenney_hexagon-kit/Models/GLB format/unit-house.glb"));
    models.castle = server.load(GltfAssetLabel::Scene(0).from_asset("kenney_hexagon-kit/Models/GLB format/unit-wall-tower.glb"));
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