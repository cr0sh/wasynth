use crate::wasm_types::TableType;

#[derive(Clone, Debug)]
pub struct TableSection {
    pub(in crate::synth) tables: Vec<TableType>,
}

impl TableSection {
    pub fn tables(&self) -> &[TableType] {
        self.tables.as_ref()
    }

    pub fn tables_mut(&mut self) -> &mut Vec<TableType> {
        &mut self.tables
    }
}
