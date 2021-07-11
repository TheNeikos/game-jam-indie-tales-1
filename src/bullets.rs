use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
};

use crate::{
    assets::GameAssets,
    movement::{Movement, MovementBundle},
    player::{Player, PlayerMouse},
};

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_bullet.system());
    }
}

#[derive(Bundle)]
struct BulletBundle {
    #[bundle]
    movement_bundle: MovementBundle,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

impl BulletBundle {
    fn new(texture_atlas_handle: Handle<TextureAtlas>) -> BulletBundle {
        BulletBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(64),
                ..Default::default()
            },
            movement_bundle: MovementBundle::default(),
        }
    }

    fn with_bullet_impulse(mut self, impulse: Vec3) -> BulletBundle {
        self.movement_bundle.movements.add(Movement::new(
            "bullet_speed",
            crate::movement::MovementModifier::Impulse { impulse },
        ));

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
            let mut direction = mouse_position.translation - player.translation;
            direction.z = player.translation.z;
            info!("{:?}", player.translation);
            commands
                .spawn()
                .insert_bundle(
                    BulletBundle::new(game_assets.texture_atlas_handle.clone())
                        .with_bullet_impulse(direction.normalize() * 1000.),
                )
                .insert(Transform::from_translation(player.translation));
        }
    }
}
