use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{get_default_camera_transform, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CameraMovementController>();
    app.add_systems(
        Update,
        record_camera_movement_controller.in_set(AppSet::RecordInput),
    );

    app.register_type::<CameraMovement>();
    app.add_systems(
        Update,
        (
            apply_camera_movement,
            camera_zoom,
            // reset_camera_location,
        ).in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CameraMovementController(pub Vec3, pub Vec3);

fn record_camera_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_controller_query: Query<&mut CameraMovementController>,
) {
    let mut intent_translation = Vec3::ZERO;
    if input.pressed(KeyCode::ArrowUp) {
        intent_translation.z -= 1.0;
        intent_translation.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        intent_translation.z += 1.0;
        intent_translation.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        intent_translation.x -= 1.0;
        intent_translation.z += 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        intent_translation.x += 1.0;
        intent_translation.z -= 1.0;
    }
    let intent_translation = intent_translation.normalize_or_zero();

    for mut camera_controller in &mut camera_controller_query {
        camera_controller.0 = intent_translation;
    }
}

fn camera_zoom(
    mut evr_scroll: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Projection, With<CameraMovement>>,
) {
    for ev in evr_scroll.read() {
        if ev.y != 0.0 {
            let Projection::Orthographic(p) = camera_query.single_mut().into_inner() else { 
                return;
            };
            p.scale *=  1.0 - (ev.y * 0.125);
        }
    }
}

fn reset_camera_location(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_movement_query: Query<(&CameraMovementController, &CameraMovement, &mut Transform)>
) {
    if input.just_pressed(KeyCode::Space) {
        for (_, _, mut transform) in &mut camera_movement_query {
            *transform = get_default_camera_transform();
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraMovement {
    pub speed: f32,
}

fn apply_camera_movement(
    time: Res<Time>,
    mut camera_movement_query: Query<(&CameraMovementController, &CameraMovement, &mut Transform)>,
) {
    for (camera_controller, camera_movement, mut transform) in &mut camera_movement_query {
        let velocity = camera_movement.speed * camera_controller.0;
        transform.translation += velocity * time.delta_seconds();
    }
}
