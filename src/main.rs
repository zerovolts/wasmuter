use std::{fs::File, i64, io, io::prelude::*};

use crate::{
    encoder::{WasmEncode, WasmEncoder},
    expression::{BlockType, Expression, Instruction},
    function_type::{FunctionType, ValueType},
    limits::Limits,
    module::Module,
    section::{
        code_section::{CodeSection, Function},
        data_section::{Data, DataSection},
        element_section::{Element, ElementSection},
        export_section::{Export, ExportDescriptor, ExportSection},
        function_section::FunctionSection,
        global_section::{Global, GlobalSection},
        import_section::{Import, ImportDescriptor, ImportSection},
        memory_section::{Memory, MemorySection},
        start_section::StartSection,
        table_section::{ElementType, Table, TableSection},
        type_section::TypeSection,
        Section,
    },
};

mod constants;
mod encoder;
mod expression;
mod function_type;
mod limits;
mod module;
mod section;

fn main() -> io::Result<()> {
    use Instruction::*;
    let wasm_module = Module(vec![
        Section::TypeSection(TypeSection(vec![FunctionType::new(
            vec![ValueType::I32, ValueType::I32],
            vec![ValueType::I32],
        )])),
        Section::ImportSection(ImportSection(vec![Import::new(
            "console",
            "log",
            ImportDescriptor::TableType(Table::new(ElementType::FunctionReference, Limits::min(1))),
        )])),
        Section::FunctionSection(FunctionSection(vec![0])),
        Section::TableSection(TableSection(vec![Table::new(
            ElementType::FunctionReference,
            Limits::min(1),
        )])),
        Section::MemorySection(MemorySection(vec![Memory::new(Limits::min(1))])),
        Section::GlobalSection(GlobalSection(vec![Global::Var(
            ValueType::I32,
            Expression(vec![Instruction::I64Const(i64::max_value())]),
        )])),
        Section::ExportSection(ExportSection(vec![
            Export::new("i32_add", ExportDescriptor::FunctionIndex(0)),
            Export::new("mem", ExportDescriptor::MemoryIndex(0)),
        ])),
        Section::StartSection(StartSection(0)),
        Section::ElementSection(ElementSection(vec![Element::new(
            0,
            Expression(vec![Instruction::I32Const(0)]),
            vec![0],
        )])),
        Section::CodeSection(CodeSection(vec![Function::new(
            vec![],
            Expression(vec![
                I32Const(42),
                I32Const(42),
                I32Eq,
                IfElse(
                    BlockType::Value(ValueType::I32),
                    vec![I32Const(23), I32Const(-2), I32Add],
                    vec![I32Const(23), I32Const(-2), I32Sub],
                ),
            ]),
        )])),
        Section::DataSection(DataSection(vec![Data::new(
            0,
            Expression(vec![Instruction::I32Const(0)]),
            "hello".as_bytes().to_owned(),
        )])),
    ]);

    let mut encoder = WasmEncoder::new();
    let byte_count = wasm_module.encode(&mut encoder);

    let file_name = "output.wasm";
    let mut file = File::create(file_name)?;
    file.write_all(encoder.as_slice())?;
    println!("Wrote {} bytes to {}", byte_count, file_name);
    Ok(())
}
