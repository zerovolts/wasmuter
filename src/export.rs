use crate::{
    encoder::{WasmEncode, WasmEncoder},
    opcode::Opcode,
};

pub struct ExportSection(pub Vec<Export>);
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}
pub struct ExportDesc {
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
            encoder.push_str(name);
            encoder.push_u8(export.desc.export_type as u8);
            encoder.push_u8(export.desc.index);
            byte_count += name.len() as u8 + 3;
        }
        encoder.write_length(byte_count);
        byte_count + 2
    }
}
