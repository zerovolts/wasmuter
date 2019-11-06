use crate::{
    constants::{MAX_ABSENT, MAX_PRESENT},
    encoder::{WasmEncode, WasmEncoder},
};

pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

impl Limits {
    pub fn new(min: u32, max: Option<u32>) -> Limits {
        Limits { min, max }
    }

    pub fn min(min: u32) -> Limits {
        Limits::new(min, None)
    }

    pub fn min_max(min: u32, max: u32) -> Limits {
        Limits::new(min, Some(max))
    }
}

impl WasmEncode for Limits {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        if self.max.is_some() {
            encoder.push_u8(MAX_PRESENT);
            encoder.push_leb_u32(self.min);
            encoder.push_leb_u32(self.max.unwrap());
            3
        } else {
            encoder.push_u8(MAX_ABSENT);
            encoder.push_leb_u32(self.min);
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_encoding_without_max() {
        assert_encoding_eq(
            Limits::min(1),
            &[
                0x00, // max flag (off)
                0x01, // min
            ],
        );
    }

    #[test]
    fn test_encoding_with_max() {
        assert_encoding_eq(
            Limits::min_max(0, 1),
            &[
                0x01, // max flag (on)
                0x00, // min
                0x01, // max
            ],
        )
    }
}
