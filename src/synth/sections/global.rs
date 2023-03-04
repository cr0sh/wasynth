use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug, Default)]
pub struct SynthGlobalSection {
    pub(crate) bytes: Vec<u8>,
}

impl SynthGlobalSection {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_all(&self.bytes)?;

        wr.write_all(&[6])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}
