use std::io::{self, Write};

#[derive(Clone, Debug)]
pub struct SynthElementSection {
    pub(crate) bytes: Vec<u8>,
}

impl SynthElementSection {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_all(&self.bytes)
    }
}
