use bevy::{prelude::*, window::PrimaryWindow};

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
            reset_camera_location,
        ).in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CameraMovementController(pub Vec2, pub Vec3);

fn record_camera_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_controller_query: Query<&mut CameraMovementController>,
) {
    let mut intent_translation = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowUp) {
        intent_translation.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        intent_translation.y -= 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        intent_translation.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        intent_translation.x += 1.0;
    }
    let intent_translation = intent_translation.normalize_or_zero();

    let mut intent_rotation = Vec3::ZERO;
    if input.pressed(KeyCode::W) {
        // look down
        intent_rotation.y +=
    }

    for mut camera_controller in &mut camera_controller_query {
        camera_controller.0 = intent_translation;
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
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
    }
}