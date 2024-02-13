use std::io::prelude::*;
use xz::read::{XzEncoder, XzDecoder};

pub fn compress(content: &str) -> XzEncoder<&[u8]> {
    return XzEncoder::new(content.as_bytes(), 9);
}

pub fn decompress(data: XzEncoder<&[u8]>) -> String {
    let mut contents = String::new();
    XzDecoder::new(data)
        .read_to_string(&mut contents)
        .unwrap();
    return contents;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_test() {
        let original = String::from("Hello, World!");
        let compressed = compress(&original);
        let decompressed = decompress(compressed);
        assert_eq!(decompressed, original);
    }
}
