use crate::wasm_types::FuncType;

#[derive(Clone, Debug)]
pub struct TypeSection {
    pub(in crate::synth) types: Vec<FuncType>,
}

impl TypeSection {
    pub fn types(&self) -> &[FuncType] {
        self.types.as_ref()
    }

    pub fn types_mut(&mut self) -> &mut Vec<FuncType> {
        &mut self.types
    }
}
