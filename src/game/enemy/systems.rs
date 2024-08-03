use avian3d::prelude::{Collider, RigidBody};
use bevy::{prelude::*, transform::commands};
use rand::prelude::*;

use crate::game::spawn::level::{MAX_X, MAX_Z};

#[derive(Component)]
pub struct Enemy;

#[derive(Event, Debug)]
pub struct SpawnEnemy;

pub fn enemy_movement(
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    // TODO: Randomly move direction
}

pub fn spawn_trig(
    mut commands: Commands,
    keebs: Res<ButtonInput<KeyCode>>,
) {
    if keebs.just_pressed(KeyCode::KeyE) {
        commands.trigger(SpawnEnemy);
    }
}

pub fn spawn_enemy(
    _trig: Trigger<SpawnEnemy>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut rng = rand::thread_rng();
    let gen_x = rng.gen::<f32>() * MAX_X - MAX_X/2.0;
    let gen_z = rng.gen::<f32>() * MAX_Z - MAX_Z/2.0;
    info!("Spawned Enemy here: {gen_x}, {gen_z}");
    
    // TODO: define initial direction

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(
                StandardMaterial {
                    base_color: Color::linear_rgb(0.8, 0.1, 0.1),
                    metallic: 0.9,
                    ..default()
                }
            ),
            transform: Transform::from_xyz(gen_x, 1.0, gen_z),
            ..default()
        },
        Enemy,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0)
    ));
}