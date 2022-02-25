use bevy::prelude::*;
use bevy::sprite::Rect;
use std::mem::take;

/// Merge several texture atlases together.
/// New sprite indices ordered respective of the order of the input texture atlases.
/// ## Example
/// Three atlases: `a` with 5 sprites, `b` with 4 Sprites, and `c` with 3 sprites,
/// are merged into a single texture atlas `m`. 
/// 
/// Then the following expression is true:
/// ```
/// a.textures[0] == m.textures[0]
/// && a.textures[1] == m.textures[1]
/// && a.textures[2] == m.textures[2]
/// && a.textures[3] == m.textures[3]
/// && a.textures[4] == m.textures[4]
/// && b.textures[0] == m.textures[5]
/// && b.textures[1] == m.textures[6]
/// && b.textures[2] == m.textures[7]
/// && b.textures[3] == m.textures[8]
/// && c.textures[0] == m.textures[9]
/// && c.textures[1] == m.textures[10]
/// && c.textures[2] == m.textures[11]
/// ```
///
/// The output TextureAtlas can potentially have a lot of wasted empty space.
/// Each input atlas texture is copied whole into the ouput atlas texture.
/// 
/// Does not modify the input atlases.
/// 
/// Returns a handle to the newly created TextureAtlas asset.
pub fn merge_atlases<'a, I>(
    image_assets: &mut Assets<Image>,
    atlas_assets: &mut Assets<TextureAtlas>,
    atlas_handles: I,
) -> Handle<TextureAtlas>
where
    I: IntoIterator<Item = &'a Handle<TextureAtlas>>,
{
    let mut builder = TextureAtlasBuilder::default();
    let atlases = atlas_handles
        .into_iter()
        .map(|h| atlas_assets.get(h).expect("Missing texture atlas"))
        .collect::<Vec<_>>();
    for atlas in &atlases {
        let image = image_assets
            .get(&atlas.texture)
            .expect("Missing atlas texture");
        builder.add_texture(atlas.texture.clone_weak(), image);
    }
    let mut meta_atlas = builder.finish(image_assets).unwrap();
    let meta_rects = take(&mut meta_atlas.textures);
    let meta_handles = take(&mut meta_atlas.texture_handles).unwrap();
    for atlas in &atlases {
        let meta_rect_index = meta_handles[&atlas.texture];
        let meta_rect = meta_rects[meta_rect_index];
        for &rect in atlas.textures.iter() {
            let meta_sub_rect = Rect {
                min: meta_rect.min + rect.min,
                max: meta_rect.min + rect.max,
            };
            meta_atlas.add_texture(meta_sub_rect);
        }
    }
    atlas_assets.add(meta_atlas)
}
