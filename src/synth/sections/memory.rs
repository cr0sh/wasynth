use std::io::{self, Write};

use crate::{wasm_types::MemType, WriteExt};

#[derive(Clone, Debug)]
pub struct SynthMemorySection {
    pub(crate) memories: Vec<MemType>,
}

impl SynthMemorySection {
    pub fn memories(&self) -> &[MemType] {
        self.memories.as_ref()
    }

    pub fn memories_mut(&mut self) -> &mut Vec<MemType> {
        &mut self.memories
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_vector(&self.memories, MemType::write_into)
    }
}
