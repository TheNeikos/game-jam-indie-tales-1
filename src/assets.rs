use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets.system());
    }
}

pub struct GameAssets {
    pub entity_texture: Handle<Texture>,
    pub texture_atlas_handle: Handle<TextureAtlas>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let entity_texture = asset_server.load("entities.png");

    let texture_atlas = TextureAtlas::from_grid(
        entity_texture.clone(),
        Vec2::new(16., 16.),
        16,
        16,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(GameAssets {
        entity_texture,
        texture_atlas_handle,
    });
}