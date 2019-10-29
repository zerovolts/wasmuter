use crate::{
    constants::START_SECTION,
    encoder::{WasmEncode, WasmEncoder},
};

pub struct StartSection(pub u8);

impl WasmEncode for StartSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        encoder.push_u8(START_SECTION);
        encoder.push_u8(1); // byte count
        encoder.push_u8(self.0);
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            StartSection(255),
            &[
                0x08, // section id
                0x01, // byte count
                0xff, // function index
            ],
        );
    }
}
