use crate::{
    encoder::{WasmEncode, WasmEncoder},
    opcode::Opcode,
    section::Section,
};

pub struct Module(pub Vec<Section>);

impl WasmEncode for Module {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        let mut byte_count = 0;
        byte_count += Opcode::MagicNumber.encode(encoder);
        byte_count += Opcode::Version.encode(encoder);

        for section in self.0.iter() {
            byte_count += section.encode(encoder);
        }
        byte_count
    }
}
