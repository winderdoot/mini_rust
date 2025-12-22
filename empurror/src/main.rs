use empurror::game_logic::{events::GameEventPlugin, empire::EmpirePlugin, game_states::*, recently_moved::RecentlyMovedPlugin};
use empurror::scene::{assets::GameModelsPlugin, hex_grid::*, orbit_camera::*};
use empurror::game_systems::*;

use bevy::{prelude::*, dev_tools::fps_overlay::{FpsOverlayPlugin}, light::CascadeShadowConfigBuilder};
use empurror::ui::common::GameUIPlugin;
use std::f32::consts::{PI};


fn main() {
    App::new()
        .add_plugins(
            (
                /* Bevy built-in plugins */
                DefaultPlugins,
                // FpsOverlayPlugin::default(),
                MeshPickingPlugin,
                /* Empurror custom plugins */
                GameSystemsPlugin, /* Manages system sets and their ordering */
                GameStatePlugin,
                HexGridPlugin,
                GameModelsPlugin,
                OrbitCameraPlugin,
                GameUIPlugin,
                RecentlyMovedPlugin,
                GameEventPlugin,
                EmpirePlugin { empire_count: 5 }
            )
        )
        .add_systems(Startup, setup_light)
        .run();
}

pub fn setup_light(
    mut commands: Commands,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT / 3.0,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 100.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 10.0,
            maximum_distance: 30.0,
            ..default() 
        }
        .build(),
    ));

}