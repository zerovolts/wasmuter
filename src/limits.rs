use crate::encoder::{WasmEncode, WasmEncoder};

pub struct Limits {
    pub min: u8,
    pub max: Option<u8>,
}

impl WasmEncode for Limits {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        if self.max.is_some() {
            encoder.push_u8(1); // max flag
            encoder.push_u8(self.min);
            encoder.push_u8(self.max.unwrap());
            3
        } else {
            encoder.push_u8(0); // max flag
            encoder.push_u8(self.min);
            2
        }
    }
}
