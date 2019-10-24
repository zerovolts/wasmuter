use crate::{
    encoder::{WasmEncode, WasmEncoder},
    opcode::Opcode,
};

pub struct ExportSection(pub Vec<Export>);
pub struct Export {
    pub name: String,
    pub descriptor: ExportDescriptor,
}
pub struct ExportDescriptor {
    pub export_type: ExportType,
    pub index: u8,
}

#[derive(Copy, Clone)]
pub enum ExportType {
    FunctionIndex = 0x00,
    TableIndex = 0x01,
    MemoryIndex = 0x02,
    GlobalIndex = 0x03,
}

impl WasmEncode for ExportSection {
    fn encode(&self, encoder: &mut WasmEncoder) -> u8 {
        Opcode::ExportSection.encode(encoder);
        encoder.push_u8(0); // byte_count placeholder

        encoder.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for export in self.0.iter() {
            let name = export.name.as_str();
            encoder.push_u8(name.len() as u8);
            byte_count += encoder.push_str(name) + 3;
            encoder.push_u8(export.descriptor.export_type as u8);
            encoder.push_u8(export.descriptor.index);
        }
        encoder.write_length(byte_count);
        byte_count + 2
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
            descriptor: ExportDescriptor {
                export_type: ExportType::FunctionIndex,
                index: 255,
            },
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
