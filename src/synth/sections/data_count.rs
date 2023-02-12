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
}
