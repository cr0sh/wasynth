use crate::instructions::Expression;

#[derive(Clone, Debug)]
pub struct DataSection {
    pub(in crate::synth) all_data: Vec<Data>,
}

impl DataSection {
    pub fn all_data(&self) -> &[Data] {
        self.all_data.as_ref()
    }

    pub fn all_data_mut(&mut self) -> &mut Vec<Data> {
        &mut self.all_data
    }
}

#[derive(Clone, Debug)]
pub enum Data {
    Active {
        init: Vec<u8>,
        memory_index: u32,
        offset: Expression,
    },
    Passive(Vec<u8>),
}
