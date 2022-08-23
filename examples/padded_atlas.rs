use bevy::prelude::*;
use bevy::render::texture;
use bevy_texture_atlas_tools::*;

struct SpriteImages(Vec<Handle<Image>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    let sprite_paths = [
        "bevy_3.png",
        "blue.png",
        "green.png",
        "o.png",
        "pink.png",
        "red.png",
        "test_pattern.png",
        "x.png",
        "yellow.png",
    ];
    let sprite_images = sprite_paths
        .iter()
        .map(|sprite_path| asset_server.load(*sprite_path))
        .collect();
    commands.insert_resource(SpriteImages(sprite_images));
}

fn on_loaded(
    mut commands: Commands,
    sprite_images: Res<SpriteImages>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    use bevy::asset::LoadState;
    match asset_server.get_group_load_state(sprite_images.0.iter().map(|handle| handle.id)) {
        LoadState::Failed => {
            panic!("missing sprite asset(s)");
        }
        LoadState::Loaded => {
            let mut texture_atlas_builder = texture_atlas_padded_builder::TextureAtlasPaddedBuilder::default()
                .padding([8; 2]);
            texture_atlas_builder.add_textures(
                sprite_images.0.iter()
                .map(|handle| (
                    handle.clone(),
                    images.get(handle).expect("missing sprite asset")
                ))
            );
            let texture_atlas = texture_atlas_builder.finish(&mut images)
                .unwrap();
            commands.spawn_bundle(SpriteBundle {
                texture: texture_atlas.texture.clone(),
                ..Default::default()
            });
            atlases.add(texture_atlas);
        }
        _ => {}
    }
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::CYAN))
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(on_loaded)
    .run();
}