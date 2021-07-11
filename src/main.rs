use bevy::prelude::*;
use render::add_tile_map_graph;

mod assets;
mod bullets;
mod map;
mod misc;
mod movement;
mod player;
mod render;

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        width: 256.,
        height: 256.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(bevy_ecs_tilemap::TilemapPlugin)
    .add_plugin(bevy_webgl2::WebGL2Plugin)
    .add_plugin(map::MapPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(movement::MovementPlugin)
    .add_plugin(assets::AssetsPlugin)
    .add_plugin(bullets::BulletsPlugin);

    // app.add_system(misc::set_texture_filters_to_nearest.system());

    app.add_startup_system(setup_game.system());

    let world = app.world_mut();
    add_tile_map_graph(world);

    app.run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
