use std::{fs::File, io, io::prelude::*};

use crate::{
    encoder::{WasmEncode, WasmEncoder},
    expression::{Expression, Instruction, MemoryArguments},
    function_type::{FunctionType, ValueType},
    index::{FunctionIndex, MemoryIndex, TypeIndex},
    limits::Limits,
    module::Module,
    section::{
        code_section::{CodeSection, Function},
        data_section::{Data, DataSection},
        export_section::{Export, ExportDescriptor, ExportSection},
        function_section::FunctionSection,
        import_section::{Import, ImportDescriptor, ImportSection},
        memory_section::{Memory, MemorySection},
        type_section::TypeSection,
        Section,
    },
};

mod constants;
mod encoder;
mod expression;
mod function_type;
mod index;
mod limits;
mod module;
mod section;

fn main() -> io::Result<()> {
    let wasm_module = hello_world_example();
    let mut encoder = WasmEncoder::new();
    let byte_count = wasm_module.encode(&mut encoder);

    let file_name = "output.wasm";
    let mut file = File::create(file_name)?;
    file.write_all(encoder.as_slice())?;
    println!("Wrote {} bytes to {}", byte_count, file_name);
    Ok(())
}

fn hello_world_example() -> Module {
    use Instruction::*;
    let memory = MemoryIndex(0);
    let write_fn = FunctionIndex(0);
    let write_type = TypeIndex(0);
    let hello_world_fn = FunctionIndex(1);
    let hello_world_type = TypeIndex(1);
    Module(vec![
        Section::TypeSection(TypeSection(vec![
            FunctionType::new(
                vec![
                    ValueType::I32,
                    ValueType::I32,
                    ValueType::I32,
                    ValueType::I32,
                ],
                vec![ValueType::I32],
            ),
            FunctionType::new(vec![], vec![]),
        ])),
        Section::ImportSection(ImportSection(vec![Import::new(
            "wasi_unstable",
            "fd_write",
            ImportDescriptor::TypeIndex(write_type),
        )])),
        Section::FunctionSection(FunctionSection(vec![hello_world_type])),
        Section::MemorySection(MemorySection(vec![Memory::new(Limits::min(1))])),
        Section::ExportSection(ExportSection(vec![
            Export::new("memory", ExportDescriptor::MemoryIndex(memory)),
            Export::new("_start", ExportDescriptor::FunctionIndex(hello_world_fn)),
        ])),
        Section::CodeSection(CodeSection(vec![Function::new(
            vec![],
            Expression(vec![
                I32Const(0),
                I32Const(8),
                I32Store(MemoryArguments::new(2, 0)),
                I32Const(4),
                I32Const(12),
                I32Store(MemoryArguments::new(2, 0)),
                I32Const(1),
                I32Const(0),
                I32Const(1),
                I32Const(20),
                Call(write_fn),
                Drop,
            ]),
        )])),
        Section::DataSection(DataSection(vec![Data::new(
            memory,
            Expression(vec![I32Const(8)]),
            "hello world!\n".as_bytes().to_owned(),
        )])),
    ])
}
