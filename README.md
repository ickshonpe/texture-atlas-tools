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
