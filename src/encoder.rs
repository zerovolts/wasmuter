pub trait WasmEncode {
    /** Returns number of bytes encoded */
    fn encode(&self, encoder: &mut WasmEncoder) -> u8;
}

pub struct WasmEncoder {
    bytes: Vec<u8>,
}

impl WasmEncoder {
    pub fn new() -> Self {
        WasmEncoder { bytes: vec![] }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /**
     * Sections in Wasm require the length (in bytes) of the section to come
     * before the section data. This function allows for setting the length as
     * a placeholder value and then going back and writing in the actual length
     * once you know it.
     */
    pub fn write_length(&mut self, length: u8) {
        let len = self.bytes.len();
        self.bytes[len - (length as usize + 1)] = length;
    }

    pub fn push_u8(&mut self, byte: u8) -> u8 {
        self.bytes.push(byte);
        1
    }

    pub fn push_u32(&mut self, value: u32) -> u8 {
        for byte in value.to_le_bytes().iter() {
            self.bytes.push(*byte);
        }
        4
    }

    pub fn push_str(&mut self, string: &str) -> u8 {
        let bytestring = string.as_bytes();
        self.bytes.push(bytestring.len() as u8);
        for byte in bytestring.iter() {
            self.bytes.push(*byte);
        }
        bytestring.len() as u8 + 1
    }
}
