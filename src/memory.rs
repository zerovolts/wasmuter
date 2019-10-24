use crate::{
    encoder::{WasmEncode, WasmEncoder},
    limits::Limits,
    opcode::Opcode,
};

pub struct MemorySection(pub Vec<Memory>);
pub struct Memory {
    pub limits: Limits,
}

impl WasmEncode for MemorySection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        Opcode::MemorySection.encode(encoder);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for memory in self.0.iter() {
            byte_count += memory.encode(encoder);
        }
        encoder.write_length(byte_count);
        byte_count + 2
    }
}

impl WasmEncode for Memory {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        self.limits.encode(encoder)
    }
}
