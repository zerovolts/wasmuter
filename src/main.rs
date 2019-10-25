use std::{fs::File, io, io::prelude::*};

use crate::{
    encoder::{WasmEncode, WasmEncoder},
    function_type::{FunctionType, ValueType},
    limits::Limits,
    module::Module,
    section::{
        export_section::{Export, ExportDescriptor, ExportSection},
        function_section::FunctionSection,
        import_section::{Import, ImportDescriptor, ImportSection},
        memory_section::{Memory, MemorySection},
        table_section::{ElementType, Table, TableSection},
        type_section::TypeSection,
        Section,
    },
};

mod constants;
mod encoder;
mod function_type;
mod limits;
mod module;
mod section;

fn main() -> io::Result<()> {
    let wasm_module = Module(vec![
        Section::TypeSection(TypeSection(vec![FunctionType(
            vec![ValueType::I32, ValueType::I32],
            vec![ValueType::I32],
        )])),
        Section::ImportSection(ImportSection(vec![Import {
            module_name: "console".to_owned(),
            name: "log".to_owned(),
            descriptor: ImportDescriptor::TableType(Table {
                element_type: ElementType::FunctionReference,
                limits: Limits { min: 1, max: None },
            }),
        }])),
        Section::FunctionSection(FunctionSection(vec![0])),
        Section::TableSection(TableSection(vec![Table {
            element_type: ElementType::FunctionReference,
            limits: Limits { min: 1, max: None },
        }])),
        Section::MemorySection(MemorySection(vec![Memory {
            limits: Limits { min: 1, max: None },
        }])),
        Section::ExportSection(ExportSection(vec![
            Export {
                name: "i32_add".to_owned(),
                descriptor: ExportDescriptor::FunctionIndex(0),
            },
            Export {
                name: "mem".to_owned(),
                descriptor: ExportDescriptor::MemoryIndex(0),
            },
        ])),
    ]);

    let mut encoder = WasmEncoder::new();
    let byte_count = wasm_module.encode(&mut encoder);

    let file_name = "output.wasm";
    let mut file = File::create(file_name)?;
    file.write_all(encoder.as_slice())?;
    println!("Wrote {} bytes to {}", byte_count, file_name);
    Ok(())
}
