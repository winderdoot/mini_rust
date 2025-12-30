use bevy::{input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll}, prelude::*};
use std::f32::consts::{FRAC_PI_2};
use std::ops::Range;
use std::cmp::*;

use crate::{game_logic::recently_moved::*, game_systems::UpdateSystems};

#[derive(Resource)]
pub struct CameraSettings {
    pan_sensitivity: f32,
    orbit_sensitivity: f32,
    zoom_sensitivity: f32,
    initial_distance: f32,
    pitch_range: Range<f32>,
    distance_range: Range<f32>,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self { 
            pan_sensitivity: 2.0, 
            orbit_sensitivity: 0.6,
            zoom_sensitivity: 3.0,
            initial_distance: 70.0,
            pitch_range: -FRAC_PI_2..-0.1,
            // pitch_range: -FRAC_PI_2..2.0,
            distance_range: 10.0..80.0,
        }
    }
}

#[derive(Component)]
#[require(Camera3d, CurrentlyMoving, RecentlyMoved)]
pub struct OrbitCamera {
    center: Vec3,
    orbit_distance: f32,
}

impl OrbitCamera {
    fn from_distance(distance: f32) -> Self {
        Self {
            center: Vec3::ZERO,
            orbit_distance: distance,
        }
    }
}

pub fn camera_system(
    mut q_camera: Single<(&mut Transform, &mut OrbitCamera, &mut CurrentlyMoving)>,
    settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let (transform, camera, moved) = &mut *q_camera;
    let orbit_scalar = 0.01;
    let pan_scalar = 7.0;
    moved.0 = false; /* Only going to register as moved if player rotates the camera */

    /* Panning */
    let forward_dir = [*transform.forward(), *transform.up()]
        .iter()
        .map(|d| Vec3::new(d.x, 0.0, d.z))
        .max_by(|v1, v2| v1.length().partial_cmp(&v2.length()).unwrap_or(Ordering::Less))
        .unwrap_or(Vec3::ZERO) /* This is effectively the same as doing .unwrap() because normalize will panic if the default is returned. Spooky ;) */
        .normalize();
    let y_axis = Vec3::new(0.0, 1.0, 0.0);
    let mut move_dir = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        move_dir += forward_dir;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        move_dir += forward_dir.rotate_axis(y_axis, -FRAC_PI_2);
    }
    if keyboard.pressed(KeyCode::KeyA) {
        move_dir += forward_dir.rotate_axis(y_axis, FRAC_PI_2);
    }
    if keyboard.pressed(KeyCode::KeyS) {
        move_dir -= forward_dir;
    }
    camera.center += move_dir.normalize_or_zero() * time.delta_secs() * settings.pan_sensitivity * pan_scalar;

    /* Zooming */
    let mut delta_zoom = - mouse_scroll.delta.y * settings.zoom_sensitivity;
    if keyboard.pressed(KeyCode::ArrowDown) {
        delta_zoom += 10.0 * time.delta_secs() * settings.zoom_sensitivity;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        delta_zoom -= 10.0 * time.delta_secs() * settings.zoom_sensitivity;
    }
    camera.orbit_distance = (camera.orbit_distance + delta_zoom).clamp(settings.distance_range.start, settings.distance_range.end);

    /* Rotation */
    if mouse_buttons.pressed(MouseButton::Right) {
        let delta = mouse_motion.delta;
        let delta_yaw = - delta.x * orbit_scalar * settings.orbit_sensitivity;
        let delta_pitch = - delta.y * orbit_scalar * settings.orbit_sensitivity;
        let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw += delta_yaw;
        pitch = (pitch + delta_pitch).clamp(settings.pitch_range.start, settings.pitch_range.end);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        moved.0  = true;
    }

    /* Translate the camera to keep it centered on set point */
    transform.translation = camera.center - (transform.forward() * camera.orbit_distance);

}

pub fn spawn_camera(mut commands: Commands, settings: Res<CameraSettings>) {
    commands.spawn((
        OrbitCamera::from_distance(settings.initial_distance),
        Transform::from_xyz(2.0, settings.initial_distance, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        // DepthPrepass,
        // OcclusionCulling
    ));
}

/* Init Plugin */

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraSettings>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_system.in_set(UpdateSystems::Camera));
    }
}