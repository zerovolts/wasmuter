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
        byte_count + 2
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_encoding() {
        let mut encoder = WasmEncoder::new();
        let import_section = ImportSection(vec![Import {
            module_name: "fs".to_owned(),
            name: "read".to_owned(),
            desc: ImportDesc::TypeIndex(255),
        }]);
        let byte_count = import_section.encode(&mut encoder);
        let expected_bytes = [
            0x02, // section id
            0x0b, // byte count
            0x01, // import count
            0x02, // module name length
            0x66, 0x73, // module name ("fs")
            0x04, // name length
            0x72, 0x65, 0x61, 0x64, // name ("read")
            0x00, // import type id
            0xff, // type index
        ];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u8);
    }
}
