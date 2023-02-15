use std::fmt::Debug;

use crate::{
    instructions::Expression,
    synth::sections::{SynthData, SynthDataSection},
    Bytes, Error,
};

#[derive(Clone, Copy)]
pub struct DataSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> DataSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> Result<SynthDataSection, Error> {
        Ok(SynthDataSection {
            all_data: self
                .all_data()?
                .map(|x| x.map(Data::into_synth))
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }

    pub fn all_data(
        &self,
    ) -> Result<impl Iterator<Item = Result<Data<'bytes>, Error>> + '_, Error> {
        self.bytes.advance_vector(Data::from_bytes)
    }
}

impl<'bytes> Debug for DataSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataSection").finish()
    }
}

#[derive(Clone, Debug)]
pub enum Data<'bytes> {
    Active {
        init: &'bytes [u8],
        memory_index: u32,
        offset: Expression,
    },
    Passive(&'bytes [u8]),
}

impl<'bytes> Data<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<(Self, &'bytes [u8]), Error> {
        let (tag, bytes) = bytes.advance_u32()?;
        match tag {
            0 => {
                let (expr, bytes) = Expression::from_bytes(bytes)?;
                let (len, bytes) = bytes.advance_u32()?;
                let (init, bytes) =
                    bytes.advance_slice(len.try_into().expect("vector overflow"))?;
                Ok((
                    Self::Active {
                        init,
                        memory_index: 0,
                        offset: expr,
                    },
                    bytes,
                ))
            }
            1 => {
                let (len, bytes) = bytes.advance_u32()?;
                let (init, bytes) =
                    bytes.advance_slice(len.try_into().expect("vector overflow"))?;
                Ok((Self::Passive(init), bytes))
            }
            2 => {
                let (memory_index, bytes) = bytes.advance_u32()?;
                let (expr, bytes) = Expression::from_bytes(bytes)?;
                let (len, bytes) = bytes.advance_u32()?;
                let (init, bytes) =
                    bytes.advance_slice(len.try_into().expect("vector overflow"))?;
                Ok((
                    Self::Active {
                        init,
                        memory_index,
                        offset: expr,
                    },
                    bytes,
                ))
            }
            _ => Err(Error::DataSectionTag(tag)),
        }
    }

    pub(crate) fn into_synth(self) -> SynthData {
        match self {
            Data::Active {
                init,
                memory_index,
                offset,
            } => SynthData::Active {
                init: init.to_owned(),
                memory_index,
                offset,
            },
            Data::Passive(bytes) => SynthData::Passive(bytes.to_owned()),
        }
    }
}
