use std::{fs::File, io, io::prelude::*};

fn main() -> io::Result<()> {
    let mut emitter = Emitter::new();
    emitter.push_opcode(Opcode::MagicNumber);
    emitter.push_opcode(Opcode::Version);
    emitter.push_section(Section::MemorySection(MemorySection(vec![Memory {
        limits: Limits { min: 1, max: None },
    }])));

    let mut file = File::create("output.wasm")?;
    file.write_all(emitter.as_slice())?;
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Opcode {
    MagicNumber,
    Version,
    MemorySection,
}

const MAGIC_NUMBER: u32 = 0x6d736100; // \0asm
const VERSION: u32 = 0x00000001;
const MEMORY_SECTION: u8 = 0x05;

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

    pub fn push_section(&mut self, section: Section) {
        let _byte_count = match section {
            Section::MemorySection(memory) => memory.emit(self),
        };
    }

    /**
     * Sections in Wasm require the length (in bytes) of the section to come
     * before the section data. This function allows for setting the length as
     * a placeholder value and then going back and writing in the actual length
     * once you know it.
     */
    pub fn write_length(&mut self, length: u8) {
        let len = self.bytes.len();
        self.bytes[len - (length as usize + 1)] = length;
    }

    pub fn push_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::MagicNumber => self.push_u32(MAGIC_NUMBER),
            Opcode::Version => self.push_u32(VERSION),
            Opcode::MemorySection => self.push_u8(MEMORY_SECTION),
        }
    }

    pub fn push_u8(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn push_u32(&mut self, value: u32) {
        for byte in value.to_le_bytes().iter() {
            self.bytes.push(*byte);
        }
    }

    pub fn push_str(&mut self, string: &str) {
        for byte in string.as_bytes().iter() {
            self.bytes.push(*byte);
        }
    }
}

trait Emit {
    /** Returns number of bytes emitted */
    fn emit(&self, emitter: &mut Emitter) -> u8;
}

enum Section {
    MemorySection(MemorySection),
}

struct MemorySection(Vec<Memory>);
struct Memory {
    limits: Limits,
}

impl Emit for MemorySection {
    fn emit(&self, emitter: &mut Emitter) -> u8 {
        emitter.push_opcode(Opcode::MemorySection);
        emitter.push_u8(0); // byte_count placeholder

        emitter.push_u8(self.0.len() as u8);
        let mut byte_count = 1;
        for memory in self.0.iter() {
            byte_count += memory.limits.emit(emitter);
        }
        emitter.write_length(byte_count);
        byte_count + 2
    }
}

struct Limits {
    min: u8,
    max: Option<u8>,
}

impl Emit for Limits {
    fn emit(&self, emitter: &mut Emitter) -> u8 {
        if self.max.is_some() {
            emitter.push_u8(1); // max flag
            emitter.push_u8(self.min);
            emitter.push_u8(self.max.unwrap());
            3
        } else {
            emitter.push_u8(0); // max flag
            emitter.push_u8(self.min);
            2
        }
    }
}
