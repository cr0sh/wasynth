use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug)]
pub struct CustomSection {
    pub(crate) name: String,
    pub(crate) bytes: Vec<u8>,
}

impl CustomSection {
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
        wr.write_name(&self.name)?;
        wr.write_all(&self.bytes)?;
        Ok(())
    }
}
