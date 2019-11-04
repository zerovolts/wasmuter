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

    // Parametric Instructions
    Drop,
    Select,

    // Variable Instructions
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    // Memory Instructions
    I32Load(MemoryArguments),
    I64Load(MemoryArguments),
    F32Load(MemoryArguments),
    F64Load(MemoryArguments),
    I32Load8S(MemoryArguments),
    I32Load8U(MemoryArguments),
    I32Load16S(MemoryArguments),
    I32Load16U(MemoryArguments),
    I64Load8S(MemoryArguments),
    I64Load8U(MemoryArguments),
    I64Load16S(MemoryArguments),
    I64Load16U(MemoryArguments),
    I64Load32S(MemoryArguments),
    I64Load32U(MemoryArguments),
    I32Store(MemoryArguments),
    I64Store(MemoryArguments),
    F32Store(MemoryArguments),
    F64Store(MemoryArguments),
    I32Store8(MemoryArguments),
    I32Store16(MemoryArguments),
    I64Store8(MemoryArguments),
    I64Store16(MemoryArguments),
    I64Store32(MemoryArguments),
    MemorySize,
    MemoryGrow,

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
                encoder.push_u8(BLOCK)
                    + block_type.encode(encoder)
                    + instructions.encode(encoder)
                    + encoder.push_u8(END)
            }
            Loop(block_type, instructions) => {
                encoder.push_u8(LOOP)
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
                    + encoder.push_u8(ELSE)
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

            // Parametric Instructions
            Drop => encoder.push_u8(DROP),
            Select => encoder.push_u8(SELECT),

            // Variable Instructions
            LocalGet(local_index) => {
                encoder.push_u8(LOCAL_GET) + encoder.push_leb_u32(*local_index)
            }
            LocalSet(local_index) => {
                encoder.push_u8(LOCAL_SET) + encoder.push_leb_u32(*local_index)
            }
            LocalTee(local_index) => {
                encoder.push_u8(LOCAL_TEE) + encoder.push_leb_u32(*local_index)
            }
            GlobalGet(global_index) => {
                encoder.push_u8(GLOBAL_GET) + encoder.push_leb_u32(*global_index)
            }
            GlobalSet(global_index) => {
                encoder.push_u8(GLOBAL_SET) + encoder.push_leb_u32(*global_index)
            }

            // Memory Instructions
            I32Load(mem_args) => encoder.push_u8(I32_LOAD) + mem_args.encode(encoder),
            I64Load(mem_args) => encoder.push_u8(I64_LOAD) + mem_args.encode(encoder),
            F32Load(mem_args) => encoder.push_u8(F32_LOAD) + mem_args.encode(encoder),
            F64Load(mem_args) => encoder.push_u8(F64_LOAD) + mem_args.encode(encoder),
            I32Load8S(mem_args) => encoder.push_u8(I32_LOAD8_S) + mem_args.encode(encoder),
            I32Load8U(mem_args) => encoder.push_u8(I32_LOAD8_U) + mem_args.encode(encoder),
            I32Load16S(mem_args) => encoder.push_u8(I32_LOAD16_S) + mem_args.encode(encoder),
            I32Load16U(mem_args) => encoder.push_u8(I32_LOAD16_U) + mem_args.encode(encoder),
            I64Load8S(mem_args) => encoder.push_u8(I64_LOAD8_S) + mem_args.encode(encoder),
            I64Load8U(mem_args) => encoder.push_u8(I64_LOAD8_U) + mem_args.encode(encoder),
            I64Load16S(mem_args) => encoder.push_u8(I64_LOAD16_S) + mem_args.encode(encoder),
            I64Load16U(mem_args) => encoder.push_u8(I64_LOAD16_U) + mem_args.encode(encoder),
            I64Load32S(mem_args) => encoder.push_u8(I64_LOAD32_S) + mem_args.encode(encoder),
            I64Load32U(mem_args) => encoder.push_u8(I64_LOAD32_U) + mem_args.encode(encoder),
            I32Store(mem_args) => encoder.push_u8(I32_STORE) + mem_args.encode(encoder),
            I64Store(mem_args) => encoder.push_u8(I64_STORE) + mem_args.encode(encoder),
            F32Store(mem_args) => encoder.push_u8(F32_STORE) + mem_args.encode(encoder),
            F64Store(mem_args) => encoder.push_u8(F64_STORE) + mem_args.encode(encoder),
            I32Store8(mem_args) => encoder.push_u8(I32_STORE8) + mem_args.encode(encoder),
            I32Store16(mem_args) => encoder.push_u8(I32_STORE16) + mem_args.encode(encoder),
            I64Store8(mem_args) => encoder.push_u8(I64_STORE8) + mem_args.encode(encoder),
            I64Store16(mem_args) => encoder.push_u8(I64_STORE16) + mem_args.encode(encoder),
            I64Store32(mem_args) => encoder.push_u8(I64_STORE32) + mem_args.encode(encoder),
            MemorySize => encoder.push_u16(MEMORY_SIZE),
            MemoryGrow => encoder.push_u16(MEMORY_GROW),

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

pub struct MemoryArguments {
    offset: u32,
    align: u32,
}

impl WasmEncode for MemoryArguments {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        encoder.push_leb_u32(self.offset) + encoder.push_leb_u32(self.align)
    }
}
