use std::io;
use std::str;
use std::io::prelude::*;
use std::fs::File;
use xz::read::{XzEncoder, XzDecoder};

pub fn process_file(filepath: &str) -> io::Result<()> {
   let mut f = File::open(filepath)?;
   let mut buffer = Vec::new();

   f.read_to_end(&mut buffer)?;

   let s = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
   };
   println!("result: {}", s);

   Ok(())
}

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
    fn compression_test() {
        let original = String::from("Hello, World!");
        let compressed = compress(&original);
        let decompressed = decompress(compressed);
        assert_eq!(decompressed, original);
    }
}
