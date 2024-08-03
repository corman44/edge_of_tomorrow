//! Spawn the main level by triggering other observers.

use std::f32::consts::PI;

use avian3d::prelude::{Collider, RigidBody};
use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::game::player::systems::SpawnPlayer;
use crate::game::moneys::PlayerMoney;

pub const MAX_X: f32 = 100.0;
pub const MAX_Z: f32 = 100.0;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct MyLight;

#[derive(Component)]
pub struct PlayerMoneyText;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ground_query: Query<&Ground>,
    mylight_query: Query<&MyLight>,
    player_money_query: Query<&PlayerMoneyText>,
    player_money: Res<PlayerMoney>,
) {

    // Plane
    if ground_query.is_empty() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::default()),
                material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
                transform: Transform::from_xyz(0.0, -1.0, 0.0)
                    .with_scale(Vec3::new(MAX_X, 0.5, MAX_Z)),
                ..default()
            },
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
            Ground,
        ));
    }

    commands.trigger(SpawnPlayer);

    // Cube
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
            transform: Transform::from_xyz(1.5, 2.0, 1.5),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0)
    ));

    // SunLight
    if mylight_query.is_empty() {
        commands.spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: light_consts::lux::OVERCAST_DAY,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 2.0, 0.0),
                    rotation: Quat::from_rotation_x(-PI / 4.),
                    ..default()
                },
                cascade_shadow_config: CascadeShadowConfigBuilder {
                    first_cascade_far_bound: 4.0,
                    maximum_distance: 100.0,
                    ..default()
                }
                .into(),
                ..default()
            },
            MyLight,
        ));
    }

    // PlayerMoneyText
    if player_money_query.is_empty() {
        commands.spawn(
            NodeBundle {
                ..default()
            }
        ).with_children(|builder| {
            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
            },
            PlayerMoneyText
            ));
        });
    }
    

}
