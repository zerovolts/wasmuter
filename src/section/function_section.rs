use crate::{
    constants::FUNCTION_SECTION,
    encoder::{WasmEncode, WasmEncoder},
    index::TypeIndex,
};

pub struct FunctionSection(pub Vec<TypeIndex>);

impl WasmEncode for FunctionSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(FUNCTION_SECTION);
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        for type_index in self.0.iter() {
            byte_count += encoder.push_leb_u32(type_index.0);
        }
        encoder.write_length(byte_count) + byte_count + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            FunctionSection(vec![TypeIndex(0), TypeIndex(1)]),
            &[
                0x03, // section id
                0x03, // byte count
                0x02, // type index count
                0x00, // type index
                0x01, // type index
            ],
        );
    }
}
