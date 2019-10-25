use crate::{
    constants::{F32, F64, FUNCTION_TYPE, I32, I64},
    encoder::{WasmEncode, WasmEncoder},
};

struct FunctionType(Vec<ValueType>, Vec<ValueType>);
enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl WasmEncode for FunctionType {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_u8(FUNCTION_TYPE);
        let mut byte_count = 2;

        // params
        encoder.push_u8(self.0.len() as u8);
        for param_type in self.0.iter() {
            byte_count += param_type.encode(encoder);
        }
        // results
        encoder.push_u8(self.1.len() as u8);
        for result_type in self.1.iter() {
            byte_count += result_type.encode(encoder);
        }
        byte_count + 1
    }
}

impl WasmEncode for ValueType {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        match self {
            ValueType::I32 => encoder.push_u8(I32),
            ValueType::I64 => encoder.push_u8(I64),
            ValueType::F32 => encoder.push_u8(F32),
            ValueType::F64 => encoder.push_u8(F64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        let mut encoder = WasmEncoder::new();
        let function_type =
            FunctionType(vec![ValueType::I32, ValueType::F32], vec![ValueType::I64]);
        let byte_count = function_type.encode(&mut encoder);
        let expected_bytes = [
            0x60, // function type id
            0x02, // param count
            0x7f, // i32
            0x7d, // f32
            0x01, // result count
            0x7e, // i64
        ];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u8);
    }
}
