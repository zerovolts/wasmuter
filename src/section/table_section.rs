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
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_u8(TABLE_SECTION);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for table in self.0.iter() {
            byte_count += table.encode(encoder);
        }
        encoder.write_length(byte_count);
        byte_count + 2
    }
}

impl WasmEncode for Table {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_u8(FUNCTION_REFERENCE);
        self.limits.encode(encoder) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_encoding() {
        let mut encoder = WasmEncoder::new();
        let table_section = TableSection(vec![Table {
            element_type: ElementType::FunctionReference,
            limits: Limits { min: 1, max: None },
        }]);
        let byte_count = table_section.encode(&mut encoder);
        let expected_bytes = [
            0x04, // section id
            0x04, // byte count
            0x01, // table count
            0x70, // element type - funcref
            0x00, 0x01, // limits
        ];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u8);
    }
}
