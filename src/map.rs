use bevy::prelude::*;
use bevy_ecs_tilemap::{LayerBuilder, LayerSettings, Map, MapQuery, Tile, TileBundle};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(start_game.system());
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_map.system());
    }
}

fn start_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
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
            bevy::render::pipeline::RenderPipeline::new(crate::render::SQUARE_PIPELINE.typed()),
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
