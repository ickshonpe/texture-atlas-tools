use bevy::math::vec2;
use bevy::prelude::*;
use bevy::{reflect::TypeUuid, utils::HashMap, asset::{LoadContext, BoxedFuture, LoadedAsset, AssetLoader}, sprite::TextureAtlas};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "b00584ad-0507-44ed-a89c-e6758f3576f6"]
pub struct AtlasManifest {
    pub rects: HashMap<String, [f32; 4]>,
    pub indices: HashMap<String, usize>
}   

#[derive(Default)]
pub struct AtlasManifestLoader;

impl AssetLoader for AtlasManifestLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let manifest = ron::de::from_bytes::<AtlasManifest>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(manifest));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

pub fn make_atlas_from_manifest(
    image_handle: Handle<Image>,
    image_dimensions: Vec2,
    manifest: &mut AtlasManifest,
) -> TextureAtlas {
    let mut atlas = TextureAtlas::new_empty(
        image_handle,
        image_dimensions
    );
    for (name, &[left, right, bottom, top]) in manifest.rects.iter() {
        let sprite_rect = bevy::sprite::Rect { 
            min: vec2(left, bottom),
            max: vec2(right, top),
        };
        let index = atlas.add_texture(sprite_rect);
        manifest.indices.insert(name.clone(), index);
    }
    atlas
}