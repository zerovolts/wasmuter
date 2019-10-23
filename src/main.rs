use std::{fs::File, io, io::prelude::*};

#[derive(Clone, Copy, Debug)]
enum Opcode {
    MagicNumber,
    Version,
}

const MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

struct Emitter {
    bytes: Vec<u8>,
}

impl Emitter {
    pub fn new() -> Self {
        Emitter { bytes: vec![] }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn push_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::MagicNumber => self.push_u32(MAGIC_NUMBER),
            Opcode::Version => self.push_u32(VERSION),
        }
    }

    fn push_u8(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    fn push_u32(&mut self, value: [u8; 4]) {
        for byte in value.iter() {
            self.bytes.push(*byte);
        }
    }
}

fn main() -> io::Result<()> {
    let mut emitter = Emitter::new();
    emitter.push_opcode(Opcode::MagicNumber);
    emitter.push_opcode(Opcode::Version);

    let mut file = File::create("output.wasm")?;
    file.write_all(emitter.as_slice())?;
    Ok(())
}
