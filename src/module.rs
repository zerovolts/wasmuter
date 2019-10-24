use crate::{
    encoder::{WasmEncode, WasmEncoder},
    opcode::Opcode,
    section::Section,
};

pub struct Module(pub Vec<Section>);

impl WasmEncode for Module {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        let mut byte_count = 0;
        byte_count += Opcode::MagicNumber.encode(encoder);
        byte_count += Opcode::Version.encode(encoder);

        for section in self.0.iter() {
            byte_count += section.encode(encoder);
        }
        byte_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        let mut encoder = WasmEncoder::new();
        let module = Module(vec![]);
        let byte_count = module.encode(&mut encoder);
        let expected_bytes = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u8);
    }
}
