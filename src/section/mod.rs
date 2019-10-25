use crate::{
    encoder::{WasmEncode, WasmEncoder},
    section::{
        export_section::ExportSection, function_section::FunctionSection,
        import_section::ImportSection, memory_section::MemorySection, table_section::TableSection,
        type_section::TypeSection,
    },
};

pub mod export_section;
pub mod function_section;
pub mod import_section;
pub mod memory_section;
pub mod table_section;
pub mod type_section;

pub enum Section {
    TypeSection(TypeSection),
    ImportSection(ImportSection),
    FunctionSection(FunctionSection),
    TableSection(TableSection),
    MemorySection(MemorySection),
    ExportSection(ExportSection),
}

impl WasmEncode for Section {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        match self {
            Section::TypeSection(type_section) => type_section.encode(encoder),
            Section::ImportSection(import_section) => import_section.encode(encoder),
            Section::FunctionSection(function_section) => function_section.encode(encoder),
            Section::TableSection(table_section) => table_section.encode(encoder),
            Section::MemorySection(memory_section) => memory_section.encode(encoder),
            Section::ExportSection(export_section) => export_section.encode(encoder),
        }
    }
}
