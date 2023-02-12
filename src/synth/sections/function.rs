#[derive(Clone, Debug)]
pub struct FunctionSection {
    pub(in crate::synth) type_indices: Vec<u32>,
}

impl FunctionSection {
    pub fn type_indices(&self) -> &[u32] {
        self.type_indices.as_ref()
    }

    pub fn type_indices_mut(&mut self) -> &mut Vec<u32> {
        &mut self.type_indices
    }
}
