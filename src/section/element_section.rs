use crate::{
    constants::ELEMENT_SECTION,
    encoder::{WasmEncode, WasmEncoder},
    expression::Expression,
    index::TableIndex,
};

pub struct ElementSection(pub Vec<Element>);

impl WasmEncode for ElementSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(ELEMENT_SECTION);
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        byte_count += self.0.encode(encoder);
        encoder.write_length(byte_count) + byte_count + 1
    }
}

pub struct Element {
    pub table_index: TableIndex,
    pub offset: Expression,
    pub initializer: Vec<u32>,
}

impl Element {
    pub fn new(table_index: TableIndex, offset: Expression, initializer: Vec<u32>) -> Element {
        Element {
            table_index,
            offset,
            initializer,
        }
    }
}

impl WasmEncode for Element {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        byte_count += encoder.push_leb_u32(self.table_index.0);
        byte_count += self.offset.encode(encoder);
        byte_count += encoder.push_leb_u32(self.initializer.len() as u32);
        for function_index in self.initializer.iter() {
            byte_count += encoder.push_leb_u32(*function_index);
        }
        byte_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{encoder::assert_encoding_eq, expression::Instruction};

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            ElementSection(vec![Element::new(
                TableIndex(0),
                Expression(vec![Instruction::I32Const(0)]),
                vec![0],
            )]),
            &[
                0x09, // section id
                0x07, // byte count
                0x01, // element count
                0x00, // table index
                0x41, 0x00, 0xb,  // (i32.const 0)
                0x01, // function index count
                0x00, // function index
            ],
        );
    }
}
