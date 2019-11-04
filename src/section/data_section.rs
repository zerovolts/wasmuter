use crate::{
    constants::DATA_SECTION,
    encoder::{WasmEncode, WasmEncoder},
    expression::Expression,
};

pub struct DataSection(pub Vec<Data>);
pub struct Data {
    pub memory_index: u32,
    pub offset: Expression,
    pub initializer: Vec<u8>,
}

impl WasmEncode for DataSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(DATA_SECTION);
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        byte_count += self.0.encode(encoder);
        encoder.write_length(byte_count) + byte_count + 1
    }
}

impl WasmEncode for Data {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        byte_count += encoder.push_leb_u32(self.memory_index);
        byte_count += self.offset.encode(encoder);
        byte_count += encoder.push_leb_u32(self.initializer.len() as u32);
        for byte in self.initializer.iter() {
            byte_count += encoder.push_u8(*byte);
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
            DataSection(vec![Data {
                memory_index: 0,
                offset: Expression(vec![Instruction::I32Const(0)]),
                initializer: vec![],
            }]),
            &[
                0x0b, // section id
                0x06, // byte count
                0x01, // data count
                0x00, // memory index
                0x41, 0x00, 0xb,  // (i32.const 0)
                0x00, // byte vec length
            ],
        );
    }
}
