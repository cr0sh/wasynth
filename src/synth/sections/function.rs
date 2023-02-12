use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug)]
pub struct FunctionSection {
    pub(crate) type_indices: Vec<u32>,
}

impl FunctionSection {
    pub fn type_indices(&self) -> &[u32] {
        self.type_indices.as_ref()
    }

    pub fn type_indices_mut(&mut self) -> &mut Vec<u32> {
        &mut self.type_indices
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_vector(&self.type_indices, |x, wr| wr.write_u32(*x))
    }
}
