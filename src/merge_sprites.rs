use bevy::prelude::*;

pub trait MergeSpritesExt {
    fn merge_sprites_in_place(&mut self, min_index: usize, max_index: usize);
    fn merge_sprites(&mut self, min_index: usize, max_index: usize) -> usize;
}

impl MergeSpritesExt for TextureAtlas {
    /// Merge sprites between indices min_index and max_index into a single sprite.
    /// The new sprite replaces the sprite at min_index.
    fn merge_sprites_in_place(&mut self, min_index: usize, max_index: usize) {
        self.textures[min_index].max = self.textures[max_index].max;
    }

    /// Merge sprites between indices min_index and max_index into a single sprite.
    /// The original sprites are not modified. A new sprite is added to the texture atlas.
    /// Returns the index of the new sprite.
    fn merge_sprites(&mut self, min_index: usize, max_index: usize) -> usize {
        let rect = bevy::sprite::Rect {
            min: self.textures[min_index].min,
            max: self.textures[max_index].max,
        };
        self.add_texture(rect)
    }
}
