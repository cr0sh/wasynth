use std::fmt::Debug;

use crate::{
    instructions::Expression,
    synth::sections::{SynthGlobal, SynthGlobalSection},
    wasm_types::GlobalType,
    Bytes, Error,
};

#[derive(Clone, Copy)]
pub struct GlobalSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> GlobalSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> Result<SynthGlobalSection, Error> {
        Ok(SynthGlobalSection {
            globals: self
                .globals()?
                .map(|x| x.map(Global::into_synth))
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }

    pub fn globals(&self) -> Result<impl Iterator<Item = Result<Global, Error>> + '_, Error> {
        self.bytes.advance_vector(Global::from_bytes)
    }
}

impl<'bytes> Debug for GlobalSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalSection").finish()
    }
}

pub struct Global {
    ty: GlobalType,
    init: Expression,
}

impl Global {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (ty, bytes) = GlobalType::from_bytes(bytes)?;
        let (init, bytes) = Expression::from_bytes(bytes)?;
        Ok((Self { ty, init }, bytes))
    }

    pub(crate) fn into_synth(self) -> SynthGlobal {
        SynthGlobal {
            ty: self.ty,
            init: self.init,
        }
    }
}
