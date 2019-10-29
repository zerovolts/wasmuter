use crate::{
    constants::{MAGIC_NUMBER, VERSION},
    encoder::{WasmEncode, WasmEncoder},
    section::Section,
};

pub struct Module(pub Vec<Section>);

impl WasmEncode for Module {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        byte_count += encoder.push_u32(MAGIC_NUMBER);
        byte_count += encoder.push_u32(VERSION);

        for section in self.0.iter() {
            byte_count += section.encode(encoder);
        }
        byte_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_encoding() {
        assert_encoding_eq(
            Module(vec![]),
            &[
                0x00, 0x61, 0x73, 0x6D, // magic number "\0asm"
                0x01, 0x00, 0x00, 0x00, // version 1
            ],
        )
    }
}
