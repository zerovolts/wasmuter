use crate::{
    constants::{CONST, GLOBAL_SECTION, VAR},
    encoder::{WasmEncode, WasmEncoder},
    expression::{Expression, Instruction},
    function_type::ValueType,
};

pub struct GlobalSection(pub Vec<Global>);

impl WasmEncode for GlobalSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(GLOBAL_SECTION);
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        byte_count += self.0.encode(encoder);
        encoder.write_length(byte_count) + byte_count + 1
    }
}

pub enum Global {
    Const(ValueType, Expression),
    Var(ValueType, Expression),
}

impl WasmEncode for Global {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        match self {
            Global::Const(value_type, expr) => {
                value_type.encode(encoder) + encoder.push_u8(CONST) + expr.encode(encoder)
            }
            Global::Var(value_type, expr) => {
                value_type.encode(encoder) + encoder.push_u8(VAR) + expr.encode(encoder)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            GlobalSection(vec![Global::Const(
                ValueType::I32,
                Expression(vec![Instruction::I32Const(42)]),
            )]),
            &[
                0x06, // section id
                0x06, // byte count
                0x01, // global count
                0x7f, // value type
                0x00, // global type
                0x41, // i32.const
                0x2a, // 42
                0x0b, // end
            ],
        );
    }
}
