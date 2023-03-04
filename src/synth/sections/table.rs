use std::io::{self, Write};

use crate::{wasm_types::TableType, WriteExt};

#[derive(Clone, Debug, Default)]
pub struct SynthTableSection {
    pub(crate) tables: Vec<TableType>,
}

impl SynthTableSection {
    pub fn tables(&self) -> &[TableType] {
        self.tables.as_ref()
    }

    pub fn tables_mut(&mut self) -> &mut Vec<TableType> {
        &mut self.tables
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_vector(&self.tables, TableType::write_into)?;

        wr.write_all(&[4])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}
