use palettevec::{
    index_buffer::aligned::AlignedIndexBuffer, palette::hybrid::HybridPalette, PaletteVec,
};

/// This PaletteVec employes a HybridPalette and an AlignedIndexBuffer.
/// This is a nice tradeoff between access performance and memory usage.
///
/// HybridPalettes threshold (right now 64) means a 64 byte stack array will
/// be created if needed. If the threshold is exceeded, a heap allocation will occur
/// and the palette switches to usinf FxHashMap.
type CharPaletteVec = PaletteVec<char, HybridPalette<43, char>, AlignedIndexBuffer>;

fn main() {
    let lorem_ipsum_utf8: &'static str = include_str!("./lorem_ipsum.txt");
    let lorem_ipsum_chars = lorem_ipsum_utf8.chars().collect::<Vec<char>>();
    let mut lorem_ipsum_pv: CharPaletteVec = CharPaletteVec::new();
    for char in &lorem_ipsum_chars {
        lorem_ipsum_pv.push_ref(char);
    }
    println!("UTF-8 String size: {}", lorem_ipsum_utf8.len());
    println!(
        "Vec<char> memory usage: stack {}, heap used: {}, heap allocated: {}",
        std::mem::size_of::<Vec<char>>(),
        lorem_ipsum_chars.len() * std::mem::size_of::<char>(),
        lorem_ipsum_chars.capacity() * std::mem::size_of::<char>()
    );
    println!(
        "PaletteVec with {} unique elements and len {}, memory usage: {:?}, ",
        lorem_ipsum_pv.unique_values(),
        lorem_ipsum_pv.len(),
        lorem_ipsum_pv.memory_usage(),
    );
}
