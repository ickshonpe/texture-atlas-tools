use bevy::prelude::*;
use bevy::sprite::Rect;
use std::mem::take;

/// Merge several texture atlases together.
/// New sprite indices ordered respective of the order of the input texture atlases.
/// ## Example 
/// You merge 3 atlases A with 5 sprites, B with 4 Sprites, and C with 3 sprites
/// into a single texture atlas M, then the new sprites indices will be:
/// * A_0 -> M_0, A_1 -> M_1, .., A_4 -> M_4
/// * B_0 -> M_5, B_1 -> M_6, .., B_3  -> M_8
/// * C_0 -> M_9, C_1 -> M_10, C_2 -> M_11
/// 
/// May potentially leave a lot of empty space, as it just copies each input atlas texture
/// whole into the larger texture. 
pub fn merge_atlases<'a, I>(
    image_assets: &mut Assets<Image>,
    atlas_assets: &mut Assets<TextureAtlas>,
    atlas_handles: I,
) -> Handle<TextureAtlas> 
where 
    I: IntoIterator<Item=&'a Handle<TextureAtlas>>
{
    let mut builder = TextureAtlasBuilder::default();
    let atlases = 
        atlas_handles.into_iter()
        .map(|h| atlas_assets.get(h).expect("Missing texture atlas"))
        .collect::<Vec<_>>();
    for atlas in &atlases {
        let image = image_assets.get(&atlas.texture)
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
            let meta_sub_rect = 
                Rect {
                   min: meta_rect.min + rect.min,
                   max: meta_rect.min + rect.max
                };
            meta_atlas.add_texture(meta_sub_rect);
        }
    }
    atlas_assets.add(meta_atlas)
}
