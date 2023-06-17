use crate::instructions::Expression;

#[derive(Clone, Debug)]
pub enum Data {
    Active {
        init: Vec<u8>,
        memory_index: u32,
        offset: Expression,
    },
    Passive(Vec<u8>),
}
