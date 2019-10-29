use crate::{
    encoder::{WasmEncode, WasmEncoder},
    section::{
        code_section::CodeSection, data_section::DataSection, element_section::ElementSection,
        export_section::ExportSection, function_section::FunctionSection,
        global_section::GlobalSection, import_section::ImportSection,
        memory_section::MemorySection, start_section::StartSection, table_section::TableSection,
        type_section::TypeSection,
    },
};

pub mod code_section;
pub mod data_section;
pub mod element_section;
pub mod export_section;
pub mod function_section;
pub mod global_section;
pub mod import_section;
pub mod memory_section;
pub mod start_section;
pub mod table_section;
pub mod type_section;

pub enum Section {
    TypeSection(TypeSection),
    ImportSection(ImportSection),
    FunctionSection(FunctionSection),
    TableSection(TableSection),
    MemorySection(MemorySection),
    GlobalSection(GlobalSection),
    ExportSection(ExportSection),
    StartSection(StartSection),
    ElementSection(ElementSection),
    CodeSection(CodeSection),
    DataSection(DataSection),
}

impl WasmEncode for Section {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        match self {
            Section::TypeSection(type_section) => type_section.encode(encoder),
            Section::ImportSection(import_section) => import_section.encode(encoder),
            Section::FunctionSection(function_section) => function_section.encode(encoder),
            Section::TableSection(table_section) => table_section.encode(encoder),
            Section::MemorySection(memory_section) => memory_section.encode(encoder),
            Section::GlobalSection(global_section) => global_section.encode(encoder),
            Section::ExportSection(export_section) => export_section.encode(encoder),
            Section::StartSection(start_section) => start_section.encode(encoder),
            Section::ElementSection(element_section) => element_section.encode(encoder),
            Section::CodeSection(code_section) => code_section.encode(encoder),
            Section::DataSection(data_section) => data_section.encode(encoder),
        }
    }
}
