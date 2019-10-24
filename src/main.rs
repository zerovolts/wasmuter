use std::{fs::File, io, io::prelude::*};

use crate::{
    encoder::{WasmEncode, WasmEncoder},
    export::{Export, ExportDescriptor, ExportSection, ExportType},
    import::{Import, ImportDescriptor, ImportSection},
    limits::Limits,
    memory::{Memory, MemorySection},
    module::Module,
    section::Section,
    table::{ElementType, Table, TableSection},
};

mod encoder;
mod export;
mod import;
mod limits;
mod memory;
mod module;
mod opcode;
mod section;
mod table;

fn main() -> io::Result<()> {
    let wasm_module = Module(vec![
        Section::ImportSection(ImportSection(vec![Import {
            module_name: "console".to_owned(),
            name: "log".to_owned(),
            descriptor: ImportDescriptor::TableType(Table {
                element_type: ElementType::FunctionReference,
                limits: Limits { min: 1, max: None },
            }),
        }])),
        Section::TableSection(TableSection(vec![Table {
            element_type: ElementType::FunctionReference,
            limits: Limits { min: 1, max: None },
        }])),
        Section::MemorySection(MemorySection(vec![Memory {
            limits: Limits { min: 1, max: None },
        }])),
        Section::ExportSection(ExportSection(vec![Export {
            name: "mem".to_owned(),
            descriptor: ExportDescriptor {
                export_type: ExportType::MemoryIndex,
                index: 0,
            },
        }])),
    ]);

    let mut encoder = WasmEncoder::new();
    let byte_count = wasm_module.encode(&mut encoder);

    let file_name = "output.wasm";
    let mut file = File::create(file_name)?;
    file.write_all(encoder.as_slice())?;
    println!("Wrote {} bytes to {}", byte_count, file_name);
    Ok(())
}
