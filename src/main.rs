use bevy::{prelude::*, render::texture::FilterMode};
use bevy_ecs_tilemap::{
    LayerBuilder, LayerSettings, Map, MapQuery, Tile, TileBundle, TilemapPlugin,
};
use render::add_tile_map_graph;

mod render;

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        width: 256.,
        height: 256.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(TilemapPlugin)
    .add_system(set_texture_filters_to_nearest.system())
    // One time greet
    .add_startup_system(start_game.system())
    .add_startup_system_to_stage(StartupStage::PostStartup, setup_map.system());

    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    let world = app.world_mut();
    add_tile_map_graph(world);

    app.run();
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Texture>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.sampler.min_filter = FilterMode::Nearest;
                }
            }
            _ => (),
        }
    }
}

struct Player;

fn start_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("atlas.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let settings = LayerSettings::new(
        UVec2::new(2, 2),
        UVec2::new(8, 8),
        Vec2::new(16., 16.),
        Vec2::new(256., 256.),
    );

    let map_center = settings.get_pixel_center();

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        settings,
        0u16,
        0u16,
        Some(RenderPipelines::from_pipelines(vec![
            bevy::render::pipeline::RenderPipeline::new(render::SQUARE_PIPELINE.typed()),
        ])),
    );

    layer_builder.set_all(TileBundle::default());

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-map_center.x, -map_center.y, 0.))
        .insert(GlobalTransform::default());

    commands
        .spawn()
        .insert(Player)
        .insert_bundle((Transform::default(), GlobalTransform::default()));
}

fn setup_map(mut commands: Commands, mut map_query: MapQuery) {
    map_query.despawn_layer_tiles(&mut commands, 0u16, 0u16);
    for x in 0..16 {
        for y in 0..16 {
            let position = UVec2::new(x, y);

            let texture_index = if x == 0 || x == 15 || y == 0 || y == 15 {
                1
            } else if x == 5 && y == 4 {
                2
            } else {
                0
            };

            map_query
                .set_tile(
                    &mut commands,
                    position,
                    Tile {
                        texture_index,
                        ..Default::default()
                    },
                    0u16,
                    0u16,
                )
                .unwrap();
            map_query.notify_chunk_for_tile(position, 0u16, 0u16);
        }
    }

    info!("Setup map!");
}
