use crate::{
    encoder::{WasmEncode, WasmEncoder},
    section::{
        export::ExportSection, import::ImportSection, memory::MemorySection, r#type::TypeSection,
        table::TableSection,
    },
};

pub mod export;
pub mod import;
pub mod memory;
pub mod table;
pub mod r#type;

pub enum Section {
    TypeSection(TypeSection),
    ImportSection(ImportSection),
    TableSection(TableSection),
    MemorySection(MemorySection),
    ExportSection(ExportSection),
}

impl WasmEncode for Section {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        match self {
            Section::TypeSection(r#type) => r#type.encode(encoder),
            Section::ImportSection(import) => import.encode(encoder),
            Section::TableSection(table) => table.encode(encoder),
            Section::MemorySection(memory) => memory.encode(encoder),
            Section::ExportSection(export) => export.encode(encoder),
        }
    }
}
