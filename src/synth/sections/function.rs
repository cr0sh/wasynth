use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug, Default)]
pub struct SynthFunctionSection {
    pub(crate) type_indices: Vec<u32>,
}

impl SynthFunctionSection {
    pub fn type_indices(&self) -> &[u32] {
        self.type_indices.as_ref()
    }

    pub fn type_indices_mut(&mut self) -> &mut Vec<u32> {
        &mut self.type_indices
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_vector(&self.type_indices, |x, wr| wr.write_u32(*x))?;

        wr.write_all(&[3])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}
