use crate::{
    constants::START_SECTION,
    encoder::{WasmEncode, WasmEncoder},
};

pub struct StartSection(pub u8);

impl WasmEncode for StartSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_u8(START_SECTION);
        encoder.push_u8(1); // byte count
        encoder.push_u8(self.0);
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_encoding() {
        let mut encoder = WasmEncoder::new();
        let type_section = StartSection(255);
        let byte_count = type_section.encode(&mut encoder);
        let expected_bytes = [
            0x08, // section id
            0x01, // byte count
            0xff, // function index
        ];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u8);
    }
}
