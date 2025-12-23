use bevy::prelude::*;

use crate::{game_logic::empire::MAX_EMPIRES, game_systems::StartupSystems};

#[derive(Resource, Default)]
pub struct Models {
    pub house: Handle<Scene>,
    pub farm: Handle<Scene>,
    pub lumber_mill: Handle<Scene>,
    pub stone_mine: Handle<Scene>,
    pub gold_mine: Handle<Scene>,
    pub castle: Handle<Scene>,
}

#[derive(Resource, Default)]
pub struct EmpireAssets {
    pub flags: Vec<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct Icons {
    pub pops: Handle<Image>,
    pub grain: Handle<Image>,
    pub lumber: Handle<Image>,
    pub stone: Handle<Image>,
    pub gold: Handle<Image>,
}

fn load_empire_flags(
    server: Res<AssetServer>,
    mut assets: ResMut<EmpireAssets>
) {
    assets.flags = Vec::with_capacity(MAX_EMPIRES as usize);
    assets.flags.push(server.load("flags/Cat_Syndicate.png"));
    assets.flags.push(server.load("flags/flag2.png"));
    assets.flags.push(server.load("flags/flag3.png"));
    assets.flags.push(server.load("flags/flag4.png"));
    assets.flags.push(server.load("flags/flag5.png"));
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

fn load_icons(
    server: Res<AssetServer>, 
    mut icons: ResMut<Icons>    
) {
    icons.pops = server.load("icons/cat3.png");
    icons.grain = server.load("icons/wheat-sack.png");
    icons.lumber = server.load("icons/wood.png");
    icons.stone = server.load("icons/stone1.png");
    icons.gold = server.load("icons/coins.png");
}

/* Init Plugin */
pub struct GameModelsPlugin;

impl Plugin for GameModelsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Models>()
            .init_resource::<EmpireAssets>()
            .init_resource::<Icons>()
            .add_systems(Startup, 
                (
                    load_building_assets,
                    load_empire_flags,
                    load_icons
                ).in_set(StartupSystems::LoadAssets)
            );
    }
}