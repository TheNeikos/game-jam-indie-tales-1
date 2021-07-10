use bevy::prelude::*;
use render::add_tile_map_graph;

mod map;
mod misc;
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
    .add_plugin(map::MapPlugin);

    app.add_system(misc::set_texture_filters_to_nearest.system());

    app.add_startup_system(start_game.system());

    let world = app.world_mut();
    add_tile_map_graph(world);

    app.run();
}

struct Player;

fn start_game(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn()
        .insert(Player)
        .insert_bundle((Transform::default(), GlobalTransform::default()));
}
