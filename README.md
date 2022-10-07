# Bevy Texture Atlas Tools

Collection of tools for working with Bevy Texture Atlases.

### Version 0.6

* Disabled Bevy default features. The crate only uses `render` and `bevy_asset` now.

### Version 0.5

* Supports Bevy 0.8
* Added an example for ```TextureAtlasPaddedBuilder```.

### Version 0.4

* Supports Bevy 0.7

### Version 0.3 Updates

*   The heterogeneous texture atlas loader has been split into its own crate
https://crates.io/crates/bevy_heterogeneous_texture_atlas_loader
* Sprite merging now implemented with an extension trait 
    ```MergeSpritesExt``` on ```TextureAtlas```.
* versions <0.4 support Bevy 0.6

### Features

* Merge sprites within an atlas.
* Merge multiple ```TextureAtlas``` s into a single larger ```TextureAtlas```.
* Alternative ```TextureAtlasBuilder```, ```TextureAtlasPaddedBuilder``` that automatically adds border padding to sprites.

### Usage

To use this crate, add this line to the ```[dependencies]``` section of your project's ```Cargo.toml``` file:
```toml
bevy_texture_atlas_tools = "0.5"
```
#

### Examples
Minimal example that builds a texture atlas with 8 x 8 padding around each sprite, run with:

```
cargo run --example padded_atlas
```

### Notes
* Nothing seems to have changed wrt ```TextureAtlas``` in the Bevy 0.8 update, BTAT 0.5 just raises the Bevy version to 0.8. Added a ```TextureAtlasPaddedBuilder``` example as a sanity check and it all seems to work.

* For working with homogeneous tilesets, you should look at these crates:
    
    https://github.com/MrGVSV/bevy_tileset
    
    https://github.com/StarArawn/bevy_ecs_tilemap

    which are both really good with lots features.

* I'm not certain that there is a neeed for padding to fix texture bleeding issues any more.  

    MSAA isn't required for most 2D games and turning it off fixes most issues. Do that by adding the following to your Bevy ``App``:

    ```rust
    app.insert_resource(MSAA { samples: 1 });
    ```
    You can also look into using texture arrays instead of atlases.

* If you do need help or have any questions, don't hesitate to contact me. I'm almost always available on the Bevy discord channel

    https://discord.com/invite/bevy
