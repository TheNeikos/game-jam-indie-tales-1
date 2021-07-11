use bevy::prelude::*;
use bevy_ecs_tilemap::{Map, Tile};
use bevy_rapier2d::{
    physics::{ColliderBundle, RapierConfiguration, RigidBodyBundle},
    prelude::{
        ColliderPosition, ColliderShape, ColliderType, ContactEvent, IntersectionEvent,
        RigidBodyType,
    },
    render::ColliderDebugRender,
};

use crate::map::Wall;

pub const PHYSICS_SCALE: f32 = 16.;
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(set_rapier_settings.system());
        app.add_system(display_events.system());
        app.add_system(setup_collisions.system());
    }
}

// Set scale in rapier settings
fn set_rapier_settings(mut settings: ResMut<RapierConfiguration>) {
    settings.scale = PHYSICS_SCALE;
    settings.gravity = Vec2::ZERO.into();
}

/* A system that displays the events. */
fn display_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        info!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        info!("Received contact event: {:?}", contact_event);
    }
}

fn setup_collisions(
    mut commands: Commands,
    tile_query: Query<(Entity, &UVec2), (Added<Tile>, With<Wall>)>,
    map_query: Query<&Transform, With<Map>>,
) {
    let map_position = map_query.single().unwrap();
    for (entity, position) in tile_query.iter() {
        commands
            .entity(entity)
            .insert(ColliderBundle {
                shape: ColliderShape::cuboid(1., 1.),
                collider_type: ColliderType::Solid,
                position: ColliderPosition(
                    ((position.as_f32().extend(0.0) + map_position.translation) / PHYSICS_SCALE)
                        .into(),
                ),
                ..Default::default()
            })
            .insert(RigidBodyBundle {
                body_type: RigidBodyType::Static,
                ..Default::default()
            })
            .insert(ColliderDebugRender::with_id(2));
    }
}
