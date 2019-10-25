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
    FunctionIndex(u8),
    TableIndex(u8),
    MemoryIndex(u8),
    GlobalIndex(u8),
}

impl WasmEncode for ExportSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        encoder.push_u8(EXPORT_SECTION);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for export in self.0.iter() {
            byte_count += encoder.push_str(export.name.as_str());
            byte_count += export.descriptor.encode(encoder);
        }
        encoder.write_length(byte_count);
        byte_count + 2
    }
}

impl WasmEncode for ExportDescriptor {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        match self {
            ExportDescriptor::FunctionIndex(function_index) => {
                encoder.push_u8(FUNCTION_INDEX);
                encoder.push_u8(*function_index);
            }
            ExportDescriptor::TableIndex(table_index) => {
                encoder.push_u8(TABLE_INDEX);
                encoder.push_u8(*table_index);
            }
            ExportDescriptor::MemoryIndex(memory_index) => {
                encoder.push_u8(MEMORY_INDEX);
                encoder.push_u8(*memory_index);
            }
            ExportDescriptor::GlobalIndex(global_index) => {
                encoder.push_u8(GLOBAL_INDEX);
                encoder.push_u8(*global_index);
            }
        }
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_encoding() {
        let mut encoder = WasmEncoder::new();
        let export_section = ExportSection(vec![Export {
            name: "add".to_owned(),
            descriptor: ExportDescriptor::FunctionIndex(255),
        }]);
        let byte_count = export_section.encode(&mut encoder);
        let expected_bytes = [
            0x07, // section id
            0x07, // byte count
            0x01, // export count
            0x03, // name length
            0x61, 0x64, 0x64, // name ("add")
            0x00, // export type id
            0xff, // export index
        ];

        assert_eq!(encoder.as_slice(), expected_bytes);
        assert_eq!(byte_count, expected_bytes.len() as u8);
    }
}
