use crate::{
    encoder::{WasmEncode, WasmEncoder},
    limits::Limits,
    opcode::Opcode,
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
        Opcode::TableSection.encode(encoder);
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
        Opcode::FunctionReferenceType.encode(encoder);
        self.limits.encode(encoder) + 1
    }
}
