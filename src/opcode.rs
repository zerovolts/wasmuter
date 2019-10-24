use crate::encoder::{WasmEncode, WasmEncoder};

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    MagicNumber,
    Version,
    ImportSection,
    TableSection,
    MemorySection,
    ExportSection,
    FunctionReferenceType,
}

impl WasmEncode for Opcode {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        match self {
            Opcode::MagicNumber => encoder.push_u32(MAGIC_NUMBER),
            Opcode::Version => encoder.push_u32(VERSION),
            Opcode::ImportSection => encoder.push_u8(IMPORT_SECTION),
            Opcode::TableSection => encoder.push_u8(TABLE_SECTION),
            Opcode::MemorySection => encoder.push_u8(MEMORY_SECTION),
            Opcode::ExportSection => encoder.push_u8(EXPORT_SECTION),
            Opcode::FunctionReferenceType => encoder.push_u8(FUNCTION_REFERENCE_TYPE),
        }
    }
}

const MAGIC_NUMBER: u32 = 0x6d736100; // \0asm
const VERSION: u32 = 0x00000001;
const IMPORT_SECTION: u8 = 0x02;
const TABLE_SECTION: u8 = 0x04;
const MEMORY_SECTION: u8 = 0x05;
const EXPORT_SECTION: u8 = 0x07;
const FUNCTION_REFERENCE_TYPE: u8 = 0x70;
