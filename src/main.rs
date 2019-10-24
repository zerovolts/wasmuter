use std::{fs::File, io, io::prelude::*};

use crate::{
    encoder::{WasmEncode, WasmEncoder},
    export::{Export, ExportDesc, ExportSection, ExportType},
    import::{Import, ImportDesc, ImportSection},
    limits::Limits,
    memory::{Memory, MemorySection},
    opcode::Opcode,
    section::Section,
    table::{ElementType, Table, TableSection},
};

mod encoder;
mod export;
mod import;
mod limits;
mod memory;
mod opcode;
mod section;
mod table;

fn main() -> io::Result<()> {
    let mut encoder = WasmEncoder::new();
    Opcode::MagicNumber.encode(&mut encoder);
    Opcode::Version.encode(&mut encoder);
    Section::ImportSection(ImportSection(vec![Import {
        module_name: "console".to_owned(),
        name: "log".to_owned(),
        desc: ImportDesc::TableType(Table {
            element_type: ElementType::FunctionReference,
            limits: Limits { min: 1, max: None },
        }),
    }]))
    .encode(&mut encoder);
    Section::TableSection(TableSection(vec![Table {
        element_type: ElementType::FunctionReference,
        limits: Limits { min: 1, max: None },
    }]))
    .encode(&mut encoder);
    Section::MemorySection(MemorySection(vec![Memory {
        limits: Limits { min: 1, max: None },
    }]))
    .encode(&mut encoder);
    Section::ExportSection(ExportSection(vec![Export {
        name: "mem".to_owned(),
        desc: ExportDesc {
            export_type: ExportType::MemoryIndex,
            index: 0,
        },
    }]))
    .encode(&mut encoder);

    let mut file = File::create("output.wasm")?;
    file.write_all(encoder.as_slice())?;
    Ok(())
}
