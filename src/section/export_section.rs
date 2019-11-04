use crate::{
    constants::{EXPORT_SECTION, FUNCTION_INDEX, GLOBAL_INDEX, MEMORY_INDEX, TABLE_INDEX},
    encoder::{WasmEncode, WasmEncoder},
};

pub struct ExportSection(pub Vec<Export>);
pub struct Export {
    pub name: String,
    pub descriptor: ExportDescriptor,
}
pub enum ExportDescriptor {
    FunctionIndex(u32),
    TableIndex(u32),
    MemoryIndex(u32),
    GlobalIndex(u32),
}

impl WasmEncode for ExportSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        let mut byte_count = 0;
        encoder.push_u8(EXPORT_SECTION);
        encoder.push_u8(0); // byte_count placeholder
        byte_count += encoder.push_leb_u32(self.0.len() as u32);
        byte_count += self.0.encode(encoder);
        encoder.write_length(byte_count) + byte_count + 1
    }
}

impl WasmEncode for Export {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        encoder.push_str(self.name.as_str()) + self.descriptor.encode(encoder)
    }
}

impl WasmEncode for ExportDescriptor {
    fn encode(&self, encoder: &mut WasmEncoder) -> u32 {
        match self {
            ExportDescriptor::FunctionIndex(function_index) => {
                encoder.push_u8(FUNCTION_INDEX);
                encoder.push_leb_u32(*function_index) + 1
            }
            ExportDescriptor::TableIndex(table_index) => {
                encoder.push_u8(TABLE_INDEX);
                encoder.push_leb_u32(*table_index) + 1
            }
            ExportDescriptor::MemoryIndex(memory_index) => {
                encoder.push_u8(MEMORY_INDEX);
                encoder.push_leb_u32(*memory_index) + 1
            }
            ExportDescriptor::GlobalIndex(global_index) => {
                encoder.push_u8(GLOBAL_INDEX);
                encoder.push_leb_u32(*global_index) + 1
            }
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
            ExportSection(vec![Export {
                name: "add".to_owned(),
                descriptor: ExportDescriptor::FunctionIndex(255),
            }]),
            &[
                0x07, // section id
                0x08, // byte count
                0x01, // export count
                0x03, // name length
                0x61, 0x64, 0x64, // name ("add")
                0x00, // export type id
                0xff, 0x01, // export index (leb128 encoded)
            ],
        )
    }
}
