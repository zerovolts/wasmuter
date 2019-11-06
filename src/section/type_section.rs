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
        byte_count += self.0.encode(encoder);
        encoder.write_length(byte_count) + byte_count + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{encoder::assert_encoding_eq, function_type::ValueType};

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            TypeSection(vec![FunctionType::new(vec![ValueType::I32], vec![])]),
            &[
                0x01, // section id
                0x05, // byte count
                0x01, // function type count
                0x60, // function type id
                0x01, // param count
                0x7f, // i32
                0x00, // result count
            ],
        );
    }
}
