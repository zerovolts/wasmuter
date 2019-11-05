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
    F32Const(f32),
    F64Const(f64),

    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Clz,
    I32Ctz,
    I32PopCnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    I64Clz,
    I64Ctz,
    I64PopCnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32CopySign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign,

    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I32ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
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
            F32Const(value) => unimplemented!(),
            F64Const(value) => unimplemented!(),

            I32Eqz => encoder.push_u8(I32_EQZ),
            I32Eq => encoder.push_u8(I32_EQ),
            I32Ne => encoder.push_u8(I32_NE),
            I32LtS => encoder.push_u8(I32_LT_S),
            I32LtU => encoder.push_u8(I32_LT_U),
            I32GtS => encoder.push_u8(I32_GT_S),
            I32GtU => encoder.push_u8(I32_GT_U),
            I32LeS => encoder.push_u8(I32_LE_S),
            I32LeU => encoder.push_u8(I32_LE_U),
            I32GeS => encoder.push_u8(I32_GE_S),
            I32GeU => encoder.push_u8(I32_GE_U),

            I64Eqz => encoder.push_u8(I64_EQZ),
            I64Eq => encoder.push_u8(I64_EQ),
            I64Ne => encoder.push_u8(I64_NE),
            I64LtS => encoder.push_u8(I64_LT_S),
            I64LtU => encoder.push_u8(I64_LT_U),
            I64GtS => encoder.push_u8(I64_GT_S),
            I64GtU => encoder.push_u8(I64_GT_U),
            I64LeS => encoder.push_u8(I64_LE_S),
            I64LeU => encoder.push_u8(I64_LE_U),
            I64GeS => encoder.push_u8(I64_GE_S),
            I64GeU => encoder.push_u8(I64_GE_U),

            F32Eq => encoder.push_u8(F32_EQ),
            F32Ne => encoder.push_u8(F32_NE),
            F32Lt => encoder.push_u8(F32_LT),
            F32Gt => encoder.push_u8(F32_GT),
            F32Le => encoder.push_u8(F32_LE),
            F32Ge => encoder.push_u8(F32_GE),

            F64Eq => encoder.push_u8(F64_EQ),
            F64Ne => encoder.push_u8(F64_NE),
            F64Lt => encoder.push_u8(F64_LT),
            F64Gt => encoder.push_u8(F64_GT),
            F64Le => encoder.push_u8(F64_LE),
            F64Ge => encoder.push_u8(F64_GE),

            I32Clz => encoder.push_u8(I32_CLZ),
            I32Ctz => encoder.push_u8(I32_CTZ),
            I32PopCnt => encoder.push_u8(I32_POPCNT),
            I32Add => encoder.push_u8(I32_ADD),
            I32Sub => encoder.push_u8(I32_SUB),
            I32Mul => encoder.push_u8(I32_MUL),
            I32DivS => encoder.push_u8(I32_DIV_S),
            I32DivU => encoder.push_u8(I32_DIV_U),
            I32RemS => encoder.push_u8(I32_REM_S),
            I32RemU => encoder.push_u8(I32_REM_U),
            I32And => encoder.push_u8(I32_AND),
            I32Or => encoder.push_u8(I32_OR),
            I32Xor => encoder.push_u8(I32_XOR),
            I32Shl => encoder.push_u8(I32_SHL),
            I32ShrS => encoder.push_u8(I32_SHR_S),
            I32ShrU => encoder.push_u8(I32_SHR_U),
            I32Rotl => encoder.push_u8(I32_ROTL),
            I32Rotr => encoder.push_u8(I32_ROTR),

            I64Clz => encoder.push_u8(I64_CLZ),
            I64Ctz => encoder.push_u8(I64_CTZ),
            I64PopCnt => encoder.push_u8(I64_POPCNT),
            I64Add => encoder.push_u8(I64_ADD),
            I64Sub => encoder.push_u8(I64_SUB),
            I64Mul => encoder.push_u8(I64_MUL),
            I64DivS => encoder.push_u8(I64_DIV_S),
            I64DivU => encoder.push_u8(I64_DIV_U),
            I64RemS => encoder.push_u8(I64_REM_S),
            I64RemU => encoder.push_u8(I64_REM_U),
            I64And => encoder.push_u8(I64_AND),
            I64Or => encoder.push_u8(I64_OR),
            I64Xor => encoder.push_u8(I64_XOR),
            I64Shl => encoder.push_u8(I64_SHL),
            I64ShrS => encoder.push_u8(I64_SHR_S),
            I64ShrU => encoder.push_u8(I64_SHR_U),
            I64Rotl => encoder.push_u8(I64_ROTL),
            I64Rotr => encoder.push_u8(I64_ROTR),

            F32Abs => encoder.push_u8(F32_ABS),
            F32Neg => encoder.push_u8(F32_NEG),
            F32Ceil => encoder.push_u8(F32_CEIL),
            F32Floor => encoder.push_u8(F32_FLOOR),
            F32Trunc => encoder.push_u8(F32_TRUNC),
            F32Nearest => encoder.push_u8(F32_NEAREST),
            F32Sqrt => encoder.push_u8(F32_SQRT),
            F32Add => encoder.push_u8(F32_ADD),
            F32Sub => encoder.push_u8(F32_SUB),
            F32Mul => encoder.push_u8(F32_MUL),
            F32Div => encoder.push_u8(F32_DIV),
            F32Min => encoder.push_u8(F32_MIN),
            F32Max => encoder.push_u8(F32_MAX),
            F32CopySign => encoder.push_u8(F32_COPYSIGN),

            F64Abs => encoder.push_u8(F64_ABS),
            F64Neg => encoder.push_u8(F64_NEG),
            F64Ceil => encoder.push_u8(F64_CEIL),
            F64Floor => encoder.push_u8(F64_FLOOR),
            F64Trunc => encoder.push_u8(F64_TRUNC),
            F64Nearest => encoder.push_u8(F64_NEAREST),
            F64Sqrt => encoder.push_u8(F64_SQRT),
            F64Add => encoder.push_u8(F64_ADD),
            F64Sub => encoder.push_u8(F64_SUB),
            F64Mul => encoder.push_u8(F64_MUL),
            F64Div => encoder.push_u8(F64_DIV),
            F64Min => encoder.push_u8(F64_MIN),
            F64Max => encoder.push_u8(F64_MAX),
            F64CopySign => encoder.push_u8(F64_COPYSIGN),

            I32WrapI64 => encoder.push_u8(I32_WRAP_I64),
            I32TruncF32S => encoder.push_u8(I32_TRUNC_F32_S),
            I32TruncF32U => encoder.push_u8(I32_TRUNC_F32_U),
            I32TruncF64S => encoder.push_u8(I32_TRUNC_F64_S),
            I32TruncF64U => encoder.push_u8(I32_TRUNC_F64_U),
            I64ExtendI32S => encoder.push_u8(I64_EXTEND_I32_S),
            I64ExtendI32U => encoder.push_u8(I64_EXTEND_I32_U),
            I64TruncF32S => encoder.push_u8(I64_TRUNC_F32_S),
            I64TruncF32U => encoder.push_u8(I64_TRUNC_F32_U),
            I64TruncF64S => encoder.push_u8(I64_TRUNC_F64_S),
            I64TruncF64U => encoder.push_u8(I64_TRUNC_F64_U),
            F32ConvertI32S => encoder.push_u8(F32_CONVERT_I32_S),
            F32ConvertI32U => encoder.push_u8(F32_CONVERT_I32_U),
            F32ConvertI64S => encoder.push_u8(F32_CONVERT_I64_S),
            F32ConvertI64U => encoder.push_u8(F32_CONVERT_I64_U),
            F32DemoteF64 => encoder.push_u8(F32_DEMOTE_F64),
            F64ConvertI32S => encoder.push_u8(F64_CONVERT_I32_S),
            F64ConvertI32U => encoder.push_u8(F64_CONVERT_I32_U),
            F64ConvertI64S => encoder.push_u8(F64_CONVERT_I64_S),
            F64ConvertI64U => encoder.push_u8(F64_CONVERT_I64_U),
            F64PromoteF32 => encoder.push_u8(F64_PROMOTE_F32),
            I32ReinterpretF32 => encoder.push_u8(I32_REINTERPRET_F32),
            I64ReinterpretF64 => encoder.push_u8(I64_REINTERPRET_F64),
            F32ReinterpretI32 => encoder.push_u8(F32_REINTERPRET_I32),
            F64ReinterpretI64 => encoder.push_u8(F64_REINTERPRET_I64),
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
    pub offset: u32,
    pub align: u32,
}

impl WasmEncode for MemoryArguments {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        encoder.push_leb_u32(self.offset) + encoder.push_leb_u32(self.align)
    }
}
