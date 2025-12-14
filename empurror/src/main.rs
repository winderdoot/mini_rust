/* TODO: Figure out hexx crate and spawn a hexagonal grid  */

use empurror::scene::panorbit_camera::*;
use empurror::scene::hex_grid::*;
use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CameraSettings>()
        .init_resource::<HexGridSettings>()
        .add_systems(Startup, (setup_scene, spawn_camera, setup_grid))
        .add_systems(Update, camera_system)
        .run();
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(0.0, 8.0, 0.0),
    ));
}