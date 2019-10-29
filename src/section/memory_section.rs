use crate::{
    constants::MEMORY_SECTION,
    encoder::{WasmEncode, WasmEncoder},
    limits::Limits,
};

pub struct MemorySection(pub Vec<Memory>);
pub struct Memory {
    pub limits: Limits,
}

impl WasmEncode for MemorySection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(MEMORY_SECTION);
        encoder.push_u8(0); // byte_count placeholder

        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        for memory in self.0.iter() {
            byte_count += memory.encode(encoder);
        }
        encoder.write_length(byte_count) + byte_count + 1
    }
}

impl WasmEncode for Memory {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        self.limits.encode(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            MemorySection(vec![Memory {
                limits: Limits { min: 1, max: None },
            }]),
            &[
                0x05, // section id
                0x03, // byte count
                0x01, // memory count
                0x00, 0x01, // limits
            ],
        );
    }
}
