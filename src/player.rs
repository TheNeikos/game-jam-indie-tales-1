use std::f32::consts::PI;

use bevy::{math::Vec3Swizzles, prelude::*, render::camera::Camera};
use bevy_rapier2d::{physics::{ColliderBundle, RigidBodyBundle}, prelude::{ColliderMassProps, ColliderShape, ColliderType, RigidBodyActivation, RigidBodyType}};

use crate::{
    assets::GameAssets,
    movement::{Movement, MovementBundle, MovementModifier, Movements},
    MainCamera, WINDOW_SCALE_FACTOR,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy, Hash, SystemLabel)]
struct MouseMovementUpdate;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system());
        app.add_system(handle_movement.system());
        app.add_system(update_mouse_position.system().label(MouseMovementUpdate));
        app.add_system(look_at_player.system().after(MouseMovementUpdate));
    }
}

pub struct Player;

fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(MovementBundle::default())
        .insert(PlayerMouse {
            position: Vec2::ZERO,
        })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            activation: RigidBodyActivation::cannot_sleep(),
            body_type: RigidBodyType::KinematicPositionBased,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(0.5),
            collider_type: ColliderType::Sensor,
            ..Default::default()
        });
}

const MAX_SPEED: f32 = 256.;

fn handle_movement(
    player_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Movements,), With<Player>>,
) {
    for (mut movements,) in player_query.iter_mut() {
        let forward = Vec3::Y;
        let right = Vec3::X;
        let mut acceleration = Vec3::ZERO;
        for key in player_input.get_pressed() {
            match key {
                KeyCode::W => acceleration += forward,
                KeyCode::S => acceleration -= forward,
                KeyCode::A => acceleration -= right,
                KeyCode::D => acceleration += right,
                _ => (),
            }
        }

        acceleration *= MAX_SPEED * 2.;

        if !acceleration.is_nan() && acceleration != Vec3::ZERO {
            movements.add(Movement::new(
                "player_input",
                MovementModifier::Momentum {
                    acceleration,
                    maximal_velocity: Some(MAX_SPEED),
                    dampening: 0.000000001,
                },
            ));
        }
    }
}

fn look_at_player(mut transform_queries: Query<(&mut Transform, &PlayerMouse), With<Player>>) {
    for (mut trans, player_mouse) in transform_queries.iter_mut() {
        let dir = player_mouse.position - trans.translation.xy();
        let angle = f32::atan2(dir.y, dir.x) - PI / 2.0;
        trans.rotation = Quat::from_axis_angle(Vec3::Z, angle);
    }
}

pub struct PlayerMouse {
    position: Vec2,
}

fn update_mouse_position(
    mut mouse_input: EventReader<CursorMoved>,
    mut mouse_query: Query<(&mut PlayerMouse,), With<Player>>,
    windows: Res<Windows>,
    camera: Query<&Transform, With<MainCamera>>,
) {
    let last_movement = if let Some(movement) = mouse_input.iter().next_back() {
        movement
    } else {
        return;
    };

    let window = windows.get(last_movement.id).unwrap();
    let camera = camera.single().unwrap();

    for (mut player_mouse,) in mouse_query.iter_mut() {
        let window_size = Vec2::new(window.width(), window.height());

        let screen_pos = last_movement.position / WINDOW_SCALE_FACTOR - window_size / 2.0;

        player_mouse.position =
            (camera.compute_matrix() * screen_pos.extend(0.).extend(1.0)).into();

        info!("Position: {:?}", player_mouse.position);
    }
}
