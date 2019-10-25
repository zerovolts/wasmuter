use crate::{
    constants::FUNCTION_SECTION,
    encoder::{WasmEncode, WasmEncoder},
};

pub struct FunctionSection(pub Vec<u8>);

impl WasmEncode for FunctionSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_u8(FUNCTION_SECTION);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for type_index in self.0.iter() {
            byte_count += encoder.push_u8(*type_index);
        }
        encoder.write_length(byte_count);
        byte_count + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_encoding() {
        let mut encoder = WasmEncoder::new();
        let memory_section = FunctionSection(vec![0, 1]);
        let byte_count = memory_section.encode(&mut encoder);
        let expected_bytes = [
            0x03, // section id
            0x03, // byte count
            0x02, // type index count
            0x00, // type index
            0x01, // type index
        ];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u8);
    }
}