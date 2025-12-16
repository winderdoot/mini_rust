use empurror::scene::orbit_camera::*;
use empurror::scene::hex_grid::*;

use bevy::{prelude::*, dev_tools::fps_overlay::{FpsOverlayPlugin}, picking::pointer::PointerInteraction, light::CascadeShadowConfigBuilder};
use std::f32::consts::{PI};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FpsOverlayPlugin::default(), MeshPickingPlugin))
        .init_resource::<CameraSettings>()
        .add_systems(Startup, (setup_scene, spawn_camera, setup_hexgrid.after(load_hexgird_settings), load_hexgird_settings))
        .add_systems(Update, camera_system)
        .run();
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT / 3.0,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 100.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
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