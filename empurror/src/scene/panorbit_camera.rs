use bevy::{input::{mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll}}, prelude::*};
use std::f32::consts::{PI, TAU, FRAC_PI_2};
use std::ops::Range;

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
            pan_sensitivity: 1.0, 
            orbit_sensitivity: 1.0,
            zoom_sensitivity: 1.0,
            initial_distance: 8.0,
            pitch_range: -FRAC_PI_2..-0.1,
            distance_range: 0.5..10.0,
        }
    }
}


#[derive(Component)]
#[require(Camera3d)]
pub struct PanOrbitCamera {
    center: Vec3,
    orbit_distance: f32,
}

impl PanOrbitCamera {
    fn from_distance(distance: f32) -> Self {
        Self {
            center: Vec3::ZERO,
            orbit_distance: distance,
        }
    }
}

pub fn camera_system(
    mut q_camera: Single<(&mut Transform, &mut PanOrbitCamera)>,
    settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    keyboard: Res<ButtonInput<KeyCode>>,
    // q_window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>
) {
    let (transform, camera) = &mut *q_camera;
    // let window = q_window.single().unwrap();
    let orbit_scalar = 0.01;
    let pan_scalar = 2.0;

    /* Panning */
    let mut forward_dir = *transform.forward();
    forward_dir.y = 0.0;
    forward_dir = forward_dir.normalize();
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
    let delta_zoom = - mouse_scroll.delta.y * settings.zoom_sensitivity;
    camera.orbit_distance = (camera.orbit_distance + delta_zoom).clamp(settings.distance_range.start, settings.distance_range.end);

    /* Rotation */
    if mouse_buttons.pressed(MouseButton::Middle) {
        let delta = mouse_motion.delta;
        let delta_yaw = - delta.x * orbit_scalar * settings.orbit_sensitivity;
        let delta_pitch = - delta.y * orbit_scalar * settings.orbit_sensitivity;
        let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw = yaw + delta_yaw;
        pitch = (pitch + delta_pitch).clamp(settings.pitch_range.start, settings.pitch_range.end);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }

    /* Translate the camera to keep it centered on set point */
    transform.translation = camera.center - (transform.forward() * camera.orbit_distance);

    
    // if let Some(cursor_position) = window.cursor_position() {
    //     let window_width = window.width();
    //     let window_height = window.height();
        
    //     let edge_margin = 10.0;

    //     if cursor_position.x < edge_margin {

    //     } else if cursor_position.x > window_width - edge_margin {
            
    //     }

    //     if cursor_position.y < edge_margin {

    //     } else if cursor_position.y > window_height - edge_margin {
            
    //     }
    // }

}

pub fn spawn_camera(mut commands: Commands, settings: Res<CameraSettings>) {
    commands.spawn((
        PanOrbitCamera::from_distance(settings.initial_distance),
        Transform::from_xyz(2.0, settings.initial_distance, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
    ));
}