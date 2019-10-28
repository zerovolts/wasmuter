use crate::{
    constants::TYPE_SECTION,
    encoder::{WasmEncode, WasmEncoder},
    function_type::FunctionType,
};

pub struct TypeSection(pub Vec<FunctionType>);

impl WasmEncode for TypeSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(TYPE_SECTION);
        encoder.push_u8(0); // byte_count placeholder

        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        for function_type in self.0.iter() {
            byte_count += function_type.encode(encoder);
        }
        encoder.write_length(byte_count) + byte_count + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function_type::ValueType;

    #[test]
    fn test_section_encoding() {
        let mut encoder = WasmEncoder::new();
        let type_section = TypeSection(vec![FunctionType(vec![ValueType::I32], vec![])]);
        let byte_count = type_section.encode(&mut encoder);
        let expected_bytes = [
            0x01, // section id
            0x05, // byte count
            0x01, // function type count
            0x60, // function type id
            0x01, // param count
            0x7f, // i32
            0x00, // result count
        ];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u32)
    }
}
