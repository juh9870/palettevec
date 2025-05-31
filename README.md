# PaletteVec

[![Test Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/alexdesander/2bb1bb9e61798b07ce8eabb2f9c9dec3/raw/palettevec_test_coverage.json)](https://gist.github.com/alexdesander/2bb1bb9e61798b07ce8eabb2f9c9dec3)

**PaletteVec is a Rust data structure designed for space-efficient storage of collections containing a limited number of unique, repeated elements. It achieves this by using a palette-based encoding scheme (palette compression), similar to how indexed color images or Minecraft chunk data are stored.**

## Key Features

* **Space Efficiency:** Drastically reduces memory footprint for data with many repeated values by storing each unique value only once in a "palette" and then using compact indices to refer to them.
* **Direct Manipulation:** Allows for operations like `push`, `pop`, `set`, and `get` directly on the compressed data without needing to decompress the entire collection.
* **Iterator:** Supports iteration over its elements without decompression.
* **Customizable Backend:** Generic over `Palette` and `IndexBuffer` traits, allowing for different storage strategies (e.g., `HybridPalette` for a balance of performance and memory, `AlignedIndexBuffer` for efficient index storage).

## When to Use PaletteVec?

`PaletteVec` is particularly useful in scenarios where:

* You have a large collection of items.
* The number of unique items in the collection is relatively small compared to the total number of items.
* Memory efficiency is a critical concern.
* You need to frequently access or modify elements within the collection.

**Common Use Cases:**

* **Voxel Engines:** Storing block data in game worlds (like Minecraft).
* **Image Compression:** Representing indexed color images.
* Any application dealing with large datasets of discrete, repeating values.
* Whenever you want to store lots of repeated values that are expensive to clone.

## Trade-offs

* **Access Overhead:** Accessing elements involves an indirection (looking up the index in the palette), which can introduce a small runtime cost compared to a standard `Vec<T>`.
* **Palette Management:** For very large numbers of unique items (a large palette), the overhead of managing the palette itself might increase. The `HybridPalette` implementation helps mitigate this by switching from an array to a HashMap when the number of unique items exceeds a threshold.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
