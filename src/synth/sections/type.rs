use std::io::{self, Write};

use crate::{wasm_types::FuncType, WriteExt};

#[derive(Clone, Debug)]
pub struct TypeSection {
    pub(crate) types: Vec<FuncType>,
}

impl TypeSection {
    pub fn types(&self) -> &[FuncType] {
        self.types.as_ref()
    }

    pub fn types_mut(&mut self) -> &mut Vec<FuncType> {
        &mut self.types
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_vector(&self.types, FuncType::write_into)
    }
}
