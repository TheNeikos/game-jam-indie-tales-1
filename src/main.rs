use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{NoUserData, RapierPhysicsPlugin},
    render::RapierRenderPlugin,
};

#[cfg(target_arch = "wasm32")]
use render::add_tile_map_graph;

pub const WINDOW_SCALE_FACTOR: f32 = 4.0;

mod assets;
mod bullets;
mod map;
mod misc;
mod movement;
mod physics;
mod player;
mod render;

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        width: 256.,
        height: 256.,
        scale_factor_override: Some(WINDOW_SCALE_FACTOR as f64),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(bevy_ecs_tilemap::TilemapPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierRenderPlugin)
    .add_plugin(map::MapPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(movement::MovementPlugin)
    .add_plugin(assets::AssetsPlugin)
    .add_plugin(bullets::BulletsPlugin)
    .add_plugin(physics::PhysicsPlugin);

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        let world = app.world_mut();
        add_tile_map_graph(world);
    }

    #[cfg(not(target_arch = "wasm32"))]
    app.add_system(misc::set_texture_filters_to_nearest.system());

    app.add_startup_system(setup_game.system());

    app.run();
}

pub struct MainCamera;

fn setup_game(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(MainCamera);
}
