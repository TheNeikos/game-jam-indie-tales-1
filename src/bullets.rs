use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    math::Vec3Swizzles,
    prelude::*,
};
use bevy_rapier2d::{physics::{ColliderBundle, ColliderPositionSync, RigidBodyBundle, RigidBodyPositionSync}, prelude::{
        ColliderShape, ColliderType, RigidBodyActivation, RigidBodyPosition, RigidBodyType,
        RigidBodyVelocity,
    }, render::ColliderDebugRender};

use crate::{assets::GameAssets, physics::PHYSICS_SCALE, player::{Player, PlayerMouse}};

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_bullet.system());
    }
}

#[derive(Bundle)]
struct BulletBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[bundle]
    rigid_body_bundle: RigidBodyBundle,
    #[bundle]
    collider_bundle: ColliderBundle,
    update_from_rigid: RigidBodyPositionSync,
}

impl BulletBundle {
    fn new(texture_atlas_handle: Handle<TextureAtlas>, position: Vec3) -> BulletBundle {
        BulletBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(64),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            rigid_body_bundle: RigidBodyBundle {
                body_type: RigidBodyType::Dynamic,
                position: RigidBodyPosition {
                    position: (position / PHYSICS_SCALE) .into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider_bundle: ColliderBundle {
                shape: ColliderShape::ball(0.1),
                collider_type: ColliderType::Solid,
                ..Default::default()
            },
            update_from_rigid: RigidBodyPositionSync::Interpolated { prev_pos: None },
        }
    }

    fn with_bullet_impulse(mut self, impulse: Vec2) -> BulletBundle {
        self.rigid_body_bundle.velocity = RigidBodyVelocity {
            linvel: (impulse / PHYSICS_SCALE).into(),
            ..Default::default()
        };

        self
    }
}

fn spawn_bullet(
    mut commands: Commands,
    mut mouse_clicks: EventReader<MouseButtonInput>,
    game_assets: Res<GameAssets>,
    player_query: Query<&Transform, With<Player>>,
    mouse_query: Query<&Transform, With<PlayerMouse>>,
) {
    let player = player_query.single().unwrap();
    let mouse_position = mouse_query.single().unwrap();
    for ev in mouse_clicks.iter() {
        if ev.button == MouseButton::Left && ev.state == ElementState::Pressed {
            let direction = mouse_position.translation - player.translation;
            commands.spawn().insert_bundle(
                BulletBundle::new(game_assets.texture_atlas_handle.clone(), player.translation)
                    .with_bullet_impulse(direction.xy().normalize() * 1000.),
            ).insert(ColliderDebugRender::default());
        }
    }
}
