use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

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
pub struct CameraMovementController(pub Vec2);

fn record_camera_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_controller_query: Query<&mut CameraMovementController>,
) {
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    let intent = intent.normalize_or_zero();

    for mut camera_controller in &mut camera_controller_query {
        camera_controller.0 = intent;
    }
}

fn reset_camera_location(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_movement_query: Query<(&CameraMovementController, &CameraMovement, &mut Transform)>
) {
    if input.just_pressed(KeyCode::Space) {
        for (_, _, mut transform) in &mut camera_movement_query {
            transform.translation = Vec3::ZERO;
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