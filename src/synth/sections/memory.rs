use crate::wasm_types::MemType;

#[derive(Clone, Debug)]
pub struct MemorySection {
    pub(crate) memories: Vec<MemType>,
}

impl MemorySection {
    pub fn memories(&self) -> &[MemType] {
        self.memories.as_ref()
    }

    pub fn memories_mut(&mut self) -> &mut Vec<MemType> {
        &mut self.memories
    }
}
