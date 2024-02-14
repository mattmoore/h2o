use std::io;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use tar::Archive;
use xz::read::{XzEncoder, XzDecoder};

pub fn decompress(source: String, target: String) -> io::Result<()> {
   let tar_xz = File::open(source)?;
   let tar = XzDecoder::new(tar_xz);
   let mut archive = Archive::new(tar);
   archive.unpack(target)?;

   Ok(())
}

pub fn xz_compress(content: &str) -> XzEncoder<&[u8]> {
    return XzEncoder::new(content.as_bytes(), 9);
}

pub fn xz_decompress(data: XzEncoder<&[u8]>) -> String {
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
    fn compression_test() {
        let original = String::from("Hello, World!");
        let compressed = xz_compress(&original);
        let decompressed = xz_decompress(compressed);
        assert_eq!(decompressed, original);
    }
}
