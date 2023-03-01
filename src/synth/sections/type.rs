use std::io::{self, Write};

use crate::{wasm_types::FuncType, WriteExt};

#[derive(Clone, Debug)]
pub struct SynthTypeSection {
    pub(crate) types: Vec<FuncType>,
}

impl SynthTypeSection {
    pub fn types(&self) -> &[FuncType] {
        self.types.as_ref()
    }

    pub fn types_mut(&mut self) -> &mut Vec<FuncType> {
        &mut self.types
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_vector(&self.types, FuncType::write_into)?;

        wr.write_all(&[1])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}
