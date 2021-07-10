use bevy::prelude::*;

use crate::movement::{Movement, MovementBundle, MovementModifier, Movements, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system());
        app.add_system(handle_movement.system());
    }
}

pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("entities.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(MovementBundle::default())
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(4., 3., 2.),
            sprite: TextureAtlasSprite::new(0),
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
