use std::io::{self, Write};

use crate::{instructions::Expression, wasm_types::ReferenceType, WriteExt};

#[derive(Clone, Debug)]
pub struct SynthElementSection {
    pub(crate) elements: Vec<SynthElem>,
}

impl SynthElementSection {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_vector(&self.elements, SynthElem::write_into)?;

        wr.write_all(&[9])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SynthElem {
    pub(crate) kind: SynthElemKind,
    pub(crate) init: SynthElemInit,
    pub(crate) mode: SynthElemMode,
}

impl SynthElem {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut discriminator: u8 = 0b000;
        let mut buf = Vec::new();
        match &self.mode {
            SynthElemMode::Active { table, offset } => {
                if *table > 0 {
                    discriminator |= 0b010;
                    buf.write_u32(*table)?;
                }
                offset.write_into(&mut buf)?;
            }
            mode @ SynthElemMode::Passive | mode @ SynthElemMode::Declarative => {
                discriminator |= 0b001;
                match mode {
                    SynthElemMode::Active { .. } => unreachable!(),
                    SynthElemMode::Passive => (),
                    SynthElemMode::Declarative => {
                        discriminator |= 0b010;
                    }
                }
            }
        }

        match &self.kind {
            SynthElemKind::FuncRef => (),
            SynthElemKind::ReferenceType(rt) => rt.write_into(&mut buf)?,
        }

        match &self.init {
            SynthElemInit::FuncIndices(indices) => {
                buf.write_vector(indices, |x, wr| wr.write_u32(*x))?;
            }
            SynthElemInit::Expressions(exprs) => {
                discriminator |= 0b100;
                buf.write_vector(exprs, Expression::write_into)?;
            }
        }

        wr.write_all(&[discriminator])?;
        wr.write_all(&buf)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SynthElemKind {
    FuncRef,
    // For the sake of simplicity, we do not create ElemKindOrRefType enum so put that case here
    ReferenceType(ReferenceType),
}

#[derive(Clone, Debug)]
pub enum SynthElemInit {
    FuncIndices(Vec<u32>),
    Expressions(Vec<Expression>),
}

#[derive(Clone, Debug)]
pub enum SynthElemMode {
    Active { table: u32, offset: Expression },
    Passive,
    Declarative,
}
