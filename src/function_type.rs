use crate::{
    constants::{F32, F64, FUNCTION_TYPE, I32, I64},
    encoder::{WasmEncode, WasmEncoder},
};

pub struct FunctionType {
    pub parameters: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

impl FunctionType {
    pub fn new(parameters: Vec<ValueType>, results: Vec<ValueType>) -> FunctionType {
        FunctionType {
            parameters,
            results,
        }
    }
}

impl WasmEncode for FunctionType {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        encoder.push_u8(FUNCTION_TYPE);
        let mut byte_count = 2;

        // params
        encoder.push_leb_u32(self.parameters.len() as u32);
        for param_type in self.parameters.iter() {
            byte_count += param_type.encode(encoder);
        }
        // results
        encoder.push_leb_u32(self.results.len() as u32);
        for result_type in self.results.iter() {
            byte_count += result_type.encode(encoder);
        }
        byte_count + 1
    }
}

pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl WasmEncode for ValueType {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
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
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_encoding() {
        assert_encoding_eq(
            FunctionType::new(vec![ValueType::I32, ValueType::F32], vec![ValueType::I64]),
            &[
                0x60, // function type id
                0x02, // param count
                0x7f, // i32
                0x7d, // f32
                0x01, // result count
                0x7e, // i64
            ],
        );
    }
}
