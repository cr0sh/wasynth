use std::io::{self, Write};

use crate::{instructions::Expression, WriteExt};

#[derive(Clone, Debug)]
pub struct SynthDataSection {
    pub(crate) all_data: Vec<SynthData>,
}

impl SynthDataSection {
    pub fn all_data(&self) -> &[SynthData] {
        self.all_data.as_ref()
    }

    pub fn all_data_mut(&mut self) -> &mut Vec<SynthData> {
        &mut self.all_data
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_vector(&self.all_data, SynthData::write_into)
    }
}

#[derive(Clone, Debug)]
pub enum SynthData {
    Active {
        init: Vec<u8>,
        memory_index: u32,
        offset: Expression,
    },
    Passive(Vec<u8>),
}

impl SynthData {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        match self {
            SynthData::Active {
                init,
                memory_index,
                offset,
            } if *memory_index == 0 => {
                wr.write_u32(0)?;
                offset.write_into(wr)?;
                wr.write_all(init)?;
            }
            SynthData::Passive(init) => {
                wr.write_u32(1)?;
                wr.write_all(init)?;
            }
            SynthData::Active {
                init,
                memory_index,
                offset,
            } => {
                wr.write_u32(2)?;
                wr.write_u32(*memory_index)?;
                offset.write_into(wr)?;
                wr.write_all(init)?;
            }
        }
        Ok(())
    }
}
