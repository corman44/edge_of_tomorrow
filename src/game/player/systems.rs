//! Spawn the player.

use avian3d::prelude::{CoefficientCombine, Collider, Friction, GravityScale, Restitution, Position};
use bevy::prelude::*;
use avian3d::math::*;

use crate::screen::Screen;

use super::plugin::CharacterControllerBundle;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Event, Debug)]
pub struct SpawnBullet;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const START_POSITION: Vec3 = Vec3::new(0.0, 1.5, 0.0);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(START_POSITION.x, START_POSITION.y, START_POSITION.z),
            ..default()
        },
        CharacterControllerBundle::new(
            Collider::capsule(0.4, 1.0),
            Position(START_POSITION)
        ).with_movement(
            50.0,
            0.92,
            7.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
        StateScoped(Screen::Playing),
        Player,
    ));
}

pub fn spawn_bullet(
    _trigger: Trigger<SpawnBullet>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Position, With<Player>>
) {
    if query.is_empty() {
        return;
    }
    for player_position in query.iter() {
        info!("spawning bullet..");
        println!("Player position: {:?}", player_position.0);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.1)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_translation(player_position.0),
                ..default()
            },
        ));
    }
}