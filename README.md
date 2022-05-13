# Bevy Texture Atlas Tools

Collection of tools for working with Bevy Texture Atlases.

### Version 0.4

* Supports Bevy 0.7

### Version 0.3 Updates

*   The heterogeneous texture atlas loader has been split into its own crate
https://crates.io/crates/bevy_heterogeneous_texture_atlas_loader
* Sprite merging now implemented with an extension trait 
    ```MergeSpritesExt``` on ```TextureAtlas```.

### Features

* Merge sprites within an atlas.
* Merge multiple ```TextureAtlas``` s into a single larger ```TextureAtlas```.
* Alternative ```TextureAtlasBuilder```, ```TextureAtlasPaddedBuilder``` that automatically adds border padding to sprites.

### Usage

To use this crate, add this line to the ```[dependencies]``` section of your project's ```Cargo.toml``` file:
```toml
bevy_texture_atlas_tools = "0.4"
```
#

### Notes
* versions <0.4 support Bevy 0.6

* For working with homogeneous tilesets, you should look at these crates:
    
    https://github.com/MrGVSV/bevy_tileset
    
    https://github.com/StarArawn/bevy_ecs_tilemap

    which are both really good with lots features.

* You might not need to add padding to fix texture bleeding issues. 

    MSAA isn't required for most 2D games and turning it off fixes most issues. Do that by adding the following to your Bevy ``App``:

    ```rust
    app.insert_resource(MSAA { samples: 1 });
    ```
    You can also look into using texture arrays instead of atlases.

* There isn't proper documentation or examples, because I'm not sure that this crate is that useful. 

  If you do need help or have any questions, don't hesitate to contact me. I'm almost always available on the Bevy discord channel

    https://discord.com/invite/bevy
