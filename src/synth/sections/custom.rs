use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug)]
pub struct SynthCustomSection {
    pub(crate) name: String,
    pub(crate) bytes: Vec<u8>,
}

impl SynthCustomSection {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }

    pub(crate) fn write_into(&self, mut wr: impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_name(&self.name)?;
        buf.write_all(&self.bytes)?;

        wr.write_all(&[0])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}
