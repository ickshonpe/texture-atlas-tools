use bevy::prelude::*;

pub trait MergeSpritesExt {
    fn merge_sprites_in_place(&mut self, a: usize, b: usize);
    fn merge_sprites(&mut self, a: usize, b: usize) -> usize;
}

impl MergeSpritesExt for TextureAtlas {
    /// Creates a new sprite from the smallest rect that contains the sprites at indices `a` and `b`.
    /// The new sprite replaces the sprite at index `a`.
    fn merge_sprites_in_place(&mut self, a: usize, b: usize) {
        self.textures[a] = bevy::sprite::Rect {
            min: self.textures[a].min.min(self.textures[b].min),
            max: self.textures[a].max.max(self.textures[b].max),
        };
    }

    /// Creates a new sprite from the smallest rect that contains the sprites at indices `a` and `b`.
    /// The original sprites are not modified. A new sprite is added to the texture atlas.
    /// Returns the index of the new sprite.
    fn merge_sprites(&mut self, a: usize, b: usize) -> usize {
        self.add_texture(bevy::sprite::Rect {
            min: self.textures[a].min.min(self.textures[b].min),
            max: self.textures[a].max.max(self.textures[b].max),
        })
    }
}
