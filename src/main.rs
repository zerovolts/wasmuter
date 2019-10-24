use std::{fs::File, io, io::prelude::*};

fn main() -> io::Result<()> {
    let mut encoder = WasmEncoder::new();
    encoder.push_opcode(Opcode::MagicNumber);
    encoder.push_opcode(Opcode::Version);
    encoder.push_section(Section::TableSection(TableSection(vec![Table {
        element_type: ElementType::FunctionReference,
        limits: Limits { min: 1, max: None },
    }])));
    encoder.push_section(Section::MemorySection(MemorySection(vec![Memory {
        limits: Limits { min: 1, max: None },
    }])));
    encoder.push_section(Section::ExportSection(ExportSection(vec![Export {
        name: "mem".to_owned(),
        desc: ExportDesc {
            export_type: ExportType::Memory,
            index: 0,
        },
    }])));

    let mut file = File::create("output.wasm")?;
    file.write_all(encoder.as_slice())?;
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Opcode {
    MagicNumber,
    Version,
    TableSection,
    MemorySection,
    ExportSection,
    FunctionReferenceType,
}

const MAGIC_NUMBER: u32 = 0x6d736100; // \0asm
const VERSION: u32 = 0x00000001;
const TABLE_SECTION: u8 = 0x04;
const MEMORY_SECTION: u8 = 0x05;
const EXPORT_SECTION: u8 = 0x07;
const FUNCTION_REFERENCE_TYPE: u8 = 0x70;

struct WasmEncoder {
    bytes: Vec<u8>,
}

impl WasmEncoder {
    pub fn new() -> Self {
        WasmEncoder { bytes: vec![] }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn push_section(&mut self, section: Section) {
        let _byte_count = match section {
            Section::TableSection(table) => table.encode(self),
            Section::MemorySection(memory) => memory.encode(self),
            Section::ExportSection(export) => export.encode(self),
        };
    }

    /**
     * Sections in Wasm require the length (in bytes) of the section to come
     * before the section data. This function allows for setting the length as
     * a placeholder value and then going back and writing in the actual length
     * once you know it.
     */
    pub fn write_length(&mut self, length: u8) {
        let len = self.bytes.len();
        self.bytes[len - (length as usize + 1)] = length;
    }

    pub fn push_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::MagicNumber => self.push_u32(MAGIC_NUMBER),
            Opcode::Version => self.push_u32(VERSION),
            Opcode::TableSection => self.push_u8(TABLE_SECTION),
            Opcode::MemorySection => self.push_u8(MEMORY_SECTION),
            Opcode::ExportSection => self.push_u8(EXPORT_SECTION),
            Opcode::FunctionReferenceType => self.push_u8(FUNCTION_REFERENCE_TYPE),
        }
    }

    pub fn push_u8(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn push_u32(&mut self, value: u32) {
        for byte in value.to_le_bytes().iter() {
            self.bytes.push(*byte);
        }
    }

    pub fn push_str(&mut self, string: &str) {
        for byte in string.as_bytes().iter() {
            self.bytes.push(*byte);
        }
    }
}

trait WasmEncode {
    /** Returns number of bytes encoded */
    fn encode(&self, encoder: &mut WasmEncoder) -> u8;
}

enum Section {
    TableSection(TableSection),
    MemorySection(MemorySection),
    ExportSection(ExportSection),
}

struct TableSection(Vec<Table>);
// The Wasm spec only supports one element_type currently, so we just push that
// opcode without checking the field.
#[allow(dead_code)]
struct Table {
    element_type: ElementType,
    limits: Limits,
}
enum ElementType {
    FunctionReference,
}

impl WasmEncode for TableSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_opcode(Opcode::TableSection);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for table in self.0.iter() {
            encoder.push_opcode(Opcode::FunctionReferenceType);
            byte_count += table.limits.encode(encoder) + 1;
        }
        encoder.write_length(byte_count);
        byte_count + 2
    }
}

struct MemorySection(Vec<Memory>);
struct Memory {
    limits: Limits,
}

impl WasmEncode for MemorySection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_opcode(Opcode::MemorySection);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for memory in self.0.iter() {
            byte_count += memory.limits.encode(encoder);
        }
        encoder.write_length(byte_count);
        byte_count + 2
    }
}

struct Limits {
    min: u8,
    max: Option<u8>,
}

impl WasmEncode for Limits {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        if self.max.is_some() {
            encoder.push_u8(1); // max flag
            encoder.push_u8(self.min);
            encoder.push_u8(self.max.unwrap());
            3
        } else {
            encoder.push_u8(0); // max flag
            encoder.push_u8(self.min);
            2
        }
    }
}

struct ExportSection(Vec<Export>);
struct Export {
    name: String,
    desc: ExportDesc,
}
struct ExportDesc {
    export_type: ExportType,
    index: u8,
}

#[derive(Copy, Clone)]
enum ExportType {
    Function = 0x00,
    Table = 0x01,
    Memory = 0x02,
    Global = 0x03,
}

impl WasmEncode for ExportSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_opcode(Opcode::ExportSection);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for export in self.0.iter() {
            let name = export.name.as_str();
            encoder.push_u8(name.len() as u8);
            encoder.push_str(name);
            encoder.push_u8(export.desc.export_type as u8);
            encoder.push_u8(export.desc.index);
            byte_count += name.len() as u8 + 3;
        }
        encoder.write_length(byte_count);
        byte_count + 2
    }
}
