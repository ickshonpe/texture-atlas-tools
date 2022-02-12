use bevy::prelude::*;

pub fn merge_sprites_in_place(
    texture_atlas: &mut TextureAtlas,
    min_index: usize,
    max_index: usize,
) {
    texture_atlas.textures[min_index].max = texture_atlas.textures[max_index].max;
}

pub fn merge_sprites(
    texture_atlas: &mut TextureAtlas,
    min_index: usize,
    max_index: usize,
) -> usize {
    let rect = bevy::sprite::Rect { 
            min: texture_atlas.textures[min_index].min,
            max: texture_atlas.textures[max_index].max
        };
    texture_atlas.add_texture(rect)
}