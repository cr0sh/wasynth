use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug)]
pub struct DataCountSection {
    pub(crate) data_count: u32,
}

impl DataCountSection {
    pub fn data_count(&self) -> u32 {
        self.data_count
    }

    pub fn data_count_mut(&mut self) -> &mut u32 {
        &mut self.data_count
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_u32(self.data_count)
    }
}
