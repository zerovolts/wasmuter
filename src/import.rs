use crate::{
    encoder::{WasmEncode, WasmEncoder},
    memory::Memory,
    opcode::Opcode,
    table::Table,
};

pub struct ImportSection(pub Vec<Import>);
pub struct Import {
    pub module_name: String,
    pub name: String,
    pub desc: ImportDesc,
}
pub enum ImportDesc {
    TypeIndex(u8),
    TableType(Table),
    MemoryType(Memory),
    GlobalType, // TODO
}

impl WasmEncode for ImportSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        Opcode::ImportSection.encode(encoder);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for import in self.0.iter() {
            let module_name = import.module_name.as_str();
            encoder.push_u8(module_name.len() as u8);
            encoder.push_str(module_name);

            let name = import.name.as_str();
            encoder.push_u8(name.len() as u8);
            encoder.push_str(name);

            byte_count += import.desc.encode(encoder);
            byte_count += module_name.len() as u8 + name.len() as u8 + 2;
        }
        encoder.write_length(byte_count);
        byte_count + 3
    }
}

impl WasmEncode for ImportDesc {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        match self {
            ImportDesc::TypeIndex(type_index) => {
                encoder.push_u8(0x00);
                encoder.push_u8(*type_index);
                2
            }
            ImportDesc::TableType(table) => {
                encoder.push_u8(0x01);
                table.encode(encoder) + 1
            }
            ImportDesc::MemoryType(memory) => {
                encoder.push_u8(0x02);
                memory.encode(encoder) + 1
            }
            ImportDesc::GlobalType => 0, // TODO
        }
    }
}
