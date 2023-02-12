use std::io::{self, Write};

use crate::{wasm_types::TableType, WriteExt};

#[derive(Clone, Debug)]
pub struct TableSection {
    pub(crate) tables: Vec<TableType>,
}

impl TableSection {
    pub fn tables(&self) -> &[TableType] {
        self.tables.as_ref()
    }

    pub fn tables_mut(&mut self) -> &mut Vec<TableType> {
        &mut self.tables
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_vector(&self.tables, TableType::write_into)
    }
}
