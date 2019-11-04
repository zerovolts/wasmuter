use crate::{
    constants::{FUNCTION_REFERENCE, TABLE_SECTION},
    encoder::{WasmEncode, WasmEncoder},
    limits::Limits,
};

pub struct TableSection(pub Vec<Table>);
// The Wasm spec only supports one element_type currently, so we just push that
// opcode without checking the field.
#[allow(dead_code)]
pub struct Table {
    pub element_type: ElementType,
    pub limits: Limits,
}
pub enum ElementType {
    FunctionReference,
}

impl WasmEncode for TableSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(TABLE_SECTION);
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        byte_count += self.0.encode(encoder);
        encoder.write_length(byte_count) + byte_count + 1
    }
}

impl WasmEncode for Table {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        encoder.push_u8(FUNCTION_REFERENCE) + self.limits.encode(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            TableSection(vec![Table {
                element_type: ElementType::FunctionReference,
                limits: Limits { min: 1, max: None },
            }]),
            &[
                0x04, // section id
                0x04, // byte count
                0x01, // table count
                0x70, // element type - funcref
                0x00, 0x01, // limits
            ],
        );
    }
}
