use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug)]
pub struct SynthDataCountSection {
    pub(crate) data_count: u32,
}

impl SynthDataCountSection {
    pub fn data_count(&self) -> u32 {
        self.data_count
    }

    pub fn data_count_mut(&mut self) -> &mut u32 {
        &mut self.data_count
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_u32(self.data_count)?;

        wr.write_all(&[12])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}
