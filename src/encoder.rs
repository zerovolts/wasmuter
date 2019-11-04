use std::{f32, f64, i32, u32};

pub trait WasmEncode {
    /** Returns number of bytes encoded */
    fn encode(&self, encoder: &mut WasmEncoder) -> u32;
}

impl<T: WasmEncode> WasmEncode for Vec<T> {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        for item in self.iter() {
            byte_count += item.encode(encoder);
        }
        byte_count
    }
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
    pub fn write_length(&mut self, length: u32) -> u32 {
        let splice_index = self.bytes.len() - (length as usize + 1);
        let mut encoder = WasmEncoder::new();
        encoder.push_leb_u32(length);
        self.bytes.splice(
            splice_index..splice_index + 1,
            encoder.bytes.iter().cloned(),
        );
        encoder.bytes.len() as u32
    }

    pub fn push_u8(&mut self, byte: u8) -> u32 {
        self.bytes.push(byte);
        1
    }

    pub fn push_u16(&mut self, value: u16) -> u32 {
        for byte in value.to_le_bytes().iter() {
            self.bytes.push(*byte);
        }
        2
    }

    pub fn push_u32(&mut self, value: u32) -> u32 {
        for byte in value.to_le_bytes().iter() {
            self.bytes.push(*byte);
        }
        4
    }

    pub fn push_leb_u32(&mut self, mut value: u32) -> u32 {
        let mut byte_count = 0;
        loop {
            byte_count += 1;
            // Take 7 low order bits
            let mut byte: u8 = (value & 0x7f) as u8;
            value >>= 7;

            if value != 0 {
                // Flip high order bit to 1
                byte ^= 0x80;
                self.bytes.push(byte);
            } else {
                self.bytes.push(byte);
                break;
            }
        }
        byte_count
    }

    pub fn push_leb_u64(&mut self, mut value: u64) -> u32 {
        let mut byte_count = 0;
        loop {
            byte_count += 1;
            // Take 7 low order bits
            let mut byte: u8 = (value & 0x7f) as u8;
            value >>= 7;

            if value != 0 {
                // Flip high order bit to 1
                byte ^= 0x80;
                self.bytes.push(byte);
            } else {
                self.bytes.push(byte);
                break;
            }
        }
        byte_count
    }

    pub fn push_leb_i32(&mut self, mut value: i32) -> u32 {
        let mut byte_count = 0;
        let mut more = true;
        loop {
            byte_count += 1;
            // Take 7 low order bits
            let mut byte: u8 = (value & 0x7f) as u8;
            value >>= 7;

            let is_sign_set = byte & 0x40 > 0;
            if value == 0 && !is_sign_set || value == -1 && is_sign_set {
                more = false;
            } else {
                // Flip high order bit to 1
                byte ^= 0x80;
            }

            self.bytes.push(byte);
            if !more {
                break;
            }
        }
        byte_count
    }

    pub fn push_leb_i64(&mut self, mut value: i64) -> u32 {
        let mut byte_count = 0;
        let mut more = true;
        loop {
            byte_count += 1;
            // Take 7 low order bits
            let mut byte: u8 = (value & 0x7f) as u8;
            value >>= 7;

            let is_sign_set = byte & 0x40 > 0;
            if value == 0 && !is_sign_set || value == -1 && is_sign_set {
                more = false;
            } else {
                // Flip high order bit to 1
                byte ^= 0x80;
            }

            self.bytes.push(byte);
            if !more {
                break;
            }
        }
        byte_count
    }

    pub fn push_str(&mut self, string: &str) -> u32 {
        let bytestring = string.as_bytes();
        self.bytes.push(bytestring.len() as u8);
        for byte in bytestring.iter() {
            self.bytes.push(*byte);
        }
        bytestring.len() as u32 + 1
    }
}

pub fn assert_encoding_eq<T: WasmEncode>(item: T, expected_bytes: &[u8]) {
    let mut encoder = WasmEncoder::new();
    let byte_count = item.encode(&mut encoder);
    assert_eq!(encoder.as_slice(), expected_bytes);
    assert_eq!(byte_count, expected_bytes.len() as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leb_u32_min_encoding() {
        let mut encoder = WasmEncoder::new();
        let byte_count = encoder.push_leb_u32(u32::min_value());
        let expected_bytes = [0x00];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u32);
    }

    #[test]
    fn test_leb_u32_max_encoding() {
        let mut encoder = WasmEncoder::new();
        let byte_count = encoder.push_leb_u32(u32::max_value());
        let expected_bytes = [0xff, 0xff, 0xff, 0xff, 0x0f];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u32);
    }

    #[test]
    fn test_leb_i32_zero_encoding() {
        let mut encoder = WasmEncoder::new();
        let byte_count = encoder.push_leb_i32(0);
        let expected_bytes = [0x00];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u32);
    }

    #[test]
    fn test_leb_i32_min_encoding() {
        let mut encoder = WasmEncoder::new();
        let byte_count = encoder.push_leb_i32(i32::min_value());
        println!("{}", i32::min_value());
        let expected_bytes = [0x80, 0x80, 0x80, 0x80, 0x78];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u32);
    }

    #[test]
    fn test_leb_i32_max_encoding() {
        let mut encoder = WasmEncoder::new();
        let byte_count = encoder.push_leb_i32(i32::max_value()); //2147483647
        let expected_bytes = [0xff, 0xff, 0xff, 0xff, 0x07];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u32);
    }

    #[test]
    fn test_leb_i32_positive_encoding() {
        let mut encoder = WasmEncoder::new();
        let byte_count = encoder.push_leb_i32(64); //2147483647
        let expected_bytes = [0xc0, 0x00];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u32);
    }

    #[test]
    fn test_leb_i32_negative_encoding() {
        let mut encoder = WasmEncoder::new();
        let byte_count = encoder.push_leb_i32(-64); //2147483647
        let expected_bytes = [0x40];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u32);
    }
}
