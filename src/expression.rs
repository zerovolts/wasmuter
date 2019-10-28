use crate::{
    constants::END,
    constants::*,
    encoder::{WasmEncode, WasmEncoder},
};

pub struct Expression(pub Vec<Instruction>);

impl WasmEncode for Expression {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 1;
        for instruction in self.0.iter() {
            byte_count += instruction.encode(encoder);
        }
        encoder.push_u8(END);
        byte_count
    }
}

pub enum Instruction {
    I32Const(i32),
    I64Const(i64),
}

impl WasmEncode for Instruction {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        match self {
            Instruction::I32Const(value) => {
                encoder.push_u8(I32_CONST);
                encoder.push_leb_i32(*value) + 1
            }
            Instruction::I64Const(value) => {
                encoder.push_u8(I64_CONST);
                encoder.push_leb_i64(*value) + 1
            }
        }
    }
}
