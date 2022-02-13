# Texture Atlas Tools

Collection of tools for working with Bevy Texture Atlases.

### Features

* Merge sprites within an atlas.
* Merge multiple texture atlases into a single larger atlas.
* Create a heterogeneous texture atlas from a ron file manifest.
* Alternative TextureAtlasBuilder that automatically adds border padding to sprites.

#

### Notes

1. I don't know what I'm doing, everything is just hacked together ad hoc until it seems to work.
2. The heterogeneous loader and padded builder should probably be in crates of their own or made into a Bevy PR. They aren't because of (1).
