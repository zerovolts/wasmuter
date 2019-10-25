use crate::{
    encoder::{WasmEncode, WasmEncoder},
    section::{
        export::ExportSection, import::ImportSection, memory::MemorySection, table::TableSection,
    },
};

pub mod export;
pub mod import;
pub mod memory;
pub mod table;

pub enum Section {
    ImportSection(ImportSection),
    TableSection(TableSection),
    MemorySection(MemorySection),
    ExportSection(ExportSection),
}

impl WasmEncode for Section {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        match self {
            Section::ImportSection(import) => import.encode(encoder),
            Section::TableSection(table) => table.encode(encoder),
            Section::MemorySection(memory) => memory.encode(encoder),
            Section::ExportSection(export) => export.encode(encoder),
        }
    }
}
