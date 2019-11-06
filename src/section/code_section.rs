use crate::{
    constants::CODE_SECTION,
    encoder::{WasmEncode, WasmEncoder},
    expression::Expression,
    function_type::ValueType,
};

pub struct CodeSection(pub Vec<Function>);

impl WasmEncode for CodeSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(CODE_SECTION);
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        byte_count += self.0.encode(encoder);
        encoder.write_length(byte_count) + byte_count + 1
    }
}

pub struct Function {
    pub locals: Vec<Local>,
    pub expression: Expression,
}

impl Function {
    pub fn new(locals: Vec<Local>, expression: Expression) -> Function {
        Function { locals, expression }
    }
}

impl WasmEncode for Function {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.locals.len() as u32);
        byte_count += self.locals.encode(encoder);
        byte_count += self.expression.encode(encoder);
        encoder.write_length(byte_count) + byte_count
    }
}

pub struct Local {
    count: u32,
    value_type: ValueType,
}

impl Local {
    pub fn new(count: u32, value_type: ValueType) -> Local {
        Local { count, value_type }
    }
}

impl WasmEncode for Local {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        encoder.push_leb_u32(self.count) + self.value_type.encode(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{encoder::assert_encoding_eq, expression::Instruction};

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            CodeSection(vec![Function::new(
                vec![],
                Expression(vec![Instruction::I32Const(6)]),
            )]),
            &[
                0x0a, // section id
                0x06, // section byte count
                0x01, // function count
                0x04, // function byte count
                0x00, // local count
                0x41, 0x06, 0xb, // (i32.const 6)
            ],
        );
    }
}
