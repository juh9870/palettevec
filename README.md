# PaletteVec

**PaletteVec is space efficient data structure for storing and managing items with a limited set of repeated elements, using a palette-based encoding scheme.**

### Palette compression has the following advantages:
- Potentially insane compression ratios
- Buffer can be manipulated without decompressing
- Easy to use

### Disadvantages:
- Buffer accesses come with a runtime cost
- Large palettes (large amount of distinct items) come with a
  runtime cost of O(palette entries) per buffer access

### Use cases
Palette compression has potential to save huge amounts of memory at runtime while still allowing for manipulation of the buffer elements. Most notably, palette compression is used in minecraft for block chunk storage. For most sophisticated voxel games, palette compression is a must.

Palette compression is also used in image compression (Indexed Color Images) and audio compression.

## Example
Creating and using a `PaletteVec`:

```rust
use palettevec::PaletteVec;

fn main() {
    let mut vec = PaletteVec::new();

    // Push elements
    vec.push("apple");
    vec.push("banana");
    vec.push("apple");

    // Access elements
    assert_eq!(vec[0], "apple");
    assert_eq!(vec[1], "banana");

    // Modify elements
    vec.set(1, "cherry");
    assert_eq!(vec[1], "cherry");

    // Remove elements
    assert_eq!(vec.pop(), Some(&"apple"));

    // Iterate over elements
    for item in &vec {
        println!("{}", item);
    }
    // Optimizing the palette
    vec.optimize();
}
```

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