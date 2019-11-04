use crate::{
    constants::END,
    constants::*,
    encoder::{WasmEncode, WasmEncoder},
    function_type::ValueType,
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
    // Control Instructions
    Unreachable,
    Nop,
    Block(BlockType, Vec<Instruction>),
    Loop(BlockType, Vec<Instruction>),
    If(BlockType, Vec<Instruction>),
    IfElse(BlockType, Vec<Instruction>, Vec<Instruction>),
    Branch(u32),
    BranchIf(u32),
    BranchTable(Vec<u32>, u32),
    Return,
    Call(u32),
    CallIndirect(u32),

    // Numeric Instructions
    I32Const(i32),
    I64Const(i64),
}

impl WasmEncode for Instruction {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        use Instruction::*;
        match self {
            // Control Instructions
            Unreachable => encoder.push_u8(UNREACHABLE),
            Nop => encoder.push_u8(NOP),
            Block(block_type, instructions) => {
                encoder.push_u8(IF)
                    + block_type.encode(encoder)
                    + instructions.encode(encoder)
                    + encoder.push_u8(END)
            }
            Loop(block_type, instructions) => {
                encoder.push_u8(IF)
                    + block_type.encode(encoder)
                    + instructions.encode(encoder)
                    + encoder.push_u8(END)
            }
            If(block_type, instructions) => {
                encoder.push_u8(IF)
                    + block_type.encode(encoder)
                    + instructions.encode(encoder)
                    + encoder.push_u8(END)
            }
            IfElse(block_type, if_instr, else_instr) => {
                encoder.push_u8(IF)
                    + block_type.encode(encoder)
                    + if_instr.encode(encoder)
                    + else_instr.encode(encoder)
                    + encoder.push_u8(END)
            }
            Branch(label_index) => encoder.push_u8(BR) + encoder.push_leb_u32(*label_index),
            BranchIf(label_index) => encoder.push_u8(BR_IF) + encoder.push_leb_u32(*label_index),
            BranchTable(label_indices, label_index) => {
                let mut byte_count = 0;
                byte_count += encoder.push_u8(BR_TABLE);
                for index in label_indices {
                    byte_count += encoder.push_leb_u32(*index);
                }
                byte_count += encoder.push_leb_u32(*label_index);
                byte_count
            }
            Return => encoder.push_u8(RETURN),
            Call(function_index) => encoder.push_u8(CALL) + encoder.push_leb_u32(*function_index),
            CallIndirect(type_index) => {
                encoder.push_u8(CALL_INDIRECT) + encoder.push_leb_u32(*type_index)
            }
            // Numeric Instructions
            I32Const(value) => encoder.push_u8(I32_CONST) + encoder.push_leb_i32(*value),
            I64Const(value) => encoder.push_u8(I64_CONST) + encoder.push_leb_i64(*value),
        }
    }
}

pub enum BlockType {
    Empty,
    Value(ValueType),
}

impl WasmEncode for BlockType {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        match self {
            BlockType::Empty => encoder.push_u8(EMPTY),
            BlockType::Value(value_type) => value_type.encode(encoder),
        }
    }
}
