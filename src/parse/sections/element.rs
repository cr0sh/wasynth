use std::fmt::Debug;

use crate::{
    instructions::Expression,
    synth::sections::{
        SynthElem, SynthElemInit, SynthElemKind, SynthElemMode, SynthElementSection,
    },
    wasm_types::ReferenceType,
    Bytes, Error,
};

#[derive(Clone, Copy)]
pub struct ElementSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> ElementSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> Result<SynthElementSection, Error> {
        Ok(SynthElementSection {
            elements: self
                .elements()?
                .map(|x| x.map(Elem::into_synth))
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }

    pub fn elements(&self) -> Result<impl Iterator<Item = Result<Elem, Error>> + '_, Error> {
        self.bytes.advance_vector(Elem::from_bytes)
    }
}

impl<'bytes> Debug for ElementSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementSection").finish()
    }
}

#[derive(Clone, Debug)]
pub struct Elem {
    kind: ElemKind,
    init: ElemInit,
    mode: ElemMode,
}

impl Elem {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (discriminator, bytes) = bytes.advance_u32()?;
        let (mode, bytes) = if discriminator & 0b001 == 0 {
            let (table, bytes) = if discriminator & 0b010 == 0 {
                (0, bytes)
            } else {
                bytes.advance_u32()?
            };
            let (offset, bytes) = Expression::from_bytes(bytes)?;
            (ElemMode::Active { table, offset }, bytes)
        } else {
            let mode = if discriminator & 0b010 == 0 {
                ElemMode::Passive
            } else {
                ElemMode::Declarative
            };
            (mode, bytes)
        };

        let (kind, init, bytes) = if discriminator & 0b100 == 0 {
            let (kind, bytes) = if discriminator & 0b010 == 0 {
                (ElemKind::FuncRef, bytes)
            } else {
                ElemKind::from_bytes(bytes)?
            };

            let mut init = Vec::new();
            let mut it = bytes.advance_vector(Bytes::advance_u32)?;
            for x in &mut it {
                init.push(x?);
            }
            let bytes = it.finalize();
            (kind, ElemInit::FuncIndices(init), bytes)
        } else {
            let (&[ty], bytes) = bytes.advance()?;
            let ty = ReferenceType::from_byte(ty)?;

            let mut init = Vec::new();
            let mut it = bytes.advance_vector(Expression::from_bytes)?;
            for x in &mut it {
                init.push(x?);
            }
            let bytes = it.finalize();
            (
                ElemKind::ReferenceType(ty),
                ElemInit::Expressions(init),
                bytes,
            )
        };

        Ok((Self { kind, init, mode }, bytes))
    }

    pub(crate) fn into_synth(self) -> SynthElem {
        SynthElem {
            kind: self.kind.into_synth(),
            init: self.init.into_synth(),
            mode: self.mode.into_synth(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ElemKind {
    FuncRef,
    // For the sake of simplicity, we do not create ElemKindOrRefType enum so put that case here
    ReferenceType(ReferenceType),
}

impl ElemKind {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (&[discriminator], bytes) = bytes.advance()?;
        match discriminator {
            0x00 => Ok((Self::FuncRef, bytes)),
            other => Err(Error::ElemKind(other)),
        }
    }

    pub(crate) fn into_synth(self) -> SynthElemKind {
        match self {
            ElemKind::FuncRef => SynthElemKind::FuncRef,
            ElemKind::ReferenceType(x) => SynthElemKind::ReferenceType(x),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ElemInit {
    FuncIndices(Vec<u32>),
    Expressions(Vec<Expression>),
}

impl ElemInit {
    pub(crate) fn into_synth(self) -> SynthElemInit {
        match self {
            ElemInit::FuncIndices(x) => SynthElemInit::FuncIndices(x),
            ElemInit::Expressions(x) => SynthElemInit::Expressions(x),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ElemMode {
    Active { table: u32, offset: Expression },
    Passive,
    Declarative,
}

impl ElemMode {
    pub(crate) fn into_synth(self) -> SynthElemMode {
        match self {
            ElemMode::Active { table, offset } => SynthElemMode::Active { table, offset },
            ElemMode::Passive => SynthElemMode::Passive,
            ElemMode::Declarative => SynthElemMode::Declarative,
        }
    }
}
