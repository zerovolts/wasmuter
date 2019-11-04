use crate::{
    constants::{IMPORT_SECTION, MEMORY_TYPE, TABLE_TYPE, TYPE_INDEX},
    encoder::{WasmEncode, WasmEncoder},
    section::{memory_section::Memory, table_section::Table},
};

pub struct ImportSection(pub Vec<Import>);
pub struct Import {
    pub module_name: String,
    pub name: String,
    pub descriptor: ImportDescriptor,
}
pub enum ImportDescriptor {
    TypeIndex(u32),
    TableType(Table),
    MemoryType(Memory),
    GlobalType, // TODO
}

impl WasmEncode for ImportSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(IMPORT_SECTION);
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        byte_count += self.0.encode(encoder);
        encoder.write_length(byte_count) + byte_count + 1
    }
}

impl WasmEncode for Import {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        encoder.push_str(self.module_name.as_str())
            + encoder.push_str(self.name.as_str())
            + self.descriptor.encode(encoder)
    }
}

impl WasmEncode for ImportDescriptor {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        match self {
            ImportDescriptor::TypeIndex(type_index) => {
                encoder.push_u8(TYPE_INDEX);
                encoder.push_leb_u32(*type_index) + 1
            }
            ImportDescriptor::TableType(table) => {
                encoder.push_u8(TABLE_TYPE);
                table.encode(encoder) + 1
            }
            ImportDescriptor::MemoryType(memory) => {
                encoder.push_u8(MEMORY_TYPE);
                memory.encode(encoder) + 1
            }
            ImportDescriptor::GlobalType => 0, // TODO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::assert_encoding_eq;

    #[test]
    fn test_section_encoding() {
        assert_encoding_eq(
            ImportSection(vec![Import {
                module_name: "fs".to_owned(),
                name: "read".to_owned(),
                descriptor: ImportDescriptor::TypeIndex(255),
            }]),
            &[
                0x02, // section id
                0x0c, // byte count
                0x01, // import count
                0x02, // module name length
                0x66, 0x73, // module name ("fs")
                0x04, // name length
                0x72, 0x65, 0x61, 0x64, // name ("read")
                0x00, // import type id
                0xff, 0x01, // type index (leb128 encoded)
            ],
        );
    }
}
