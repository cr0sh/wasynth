use std::io::{self, Write};

use crate::{instructions::Expression, wasm_types::ValueType, WriteExt};

#[derive(Clone, Debug)]
pub struct SynthCodeSection {
    pub(crate) codes: Vec<SynthCode>,
}

impl SynthCodeSection {
    pub fn codes(&self) -> &[SynthCode] {
        self.codes.as_ref()
    }

    pub fn codes_mut(&mut self) -> &mut Vec<SynthCode> {
        &mut self.codes
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_vector(&self.codes, SynthCode::write_into)?;

        wr.write_all(&[10])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SynthCode {
    pub(crate) locals: Vec<ValueType>,
    pub(crate) func_expr: Expression,
}

impl SynthCode {
    pub fn func_expr(&self) -> &Expression {
        &self.func_expr
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        let mut locals = Vec::new();
        if let Some(mut ty) = self.locals.first().copied() {
            let mut cnt = 0;
            for local in &self.locals {
                if ty == *local {
                    cnt += 1;
                } else {
                    locals.push(SynthLocal { n: cnt, t: ty });
                    ty = *local;
                    cnt = 1;
                }
            }
            locals.push(SynthLocal { n: cnt, t: ty });
        }

        buf.write_vector(&locals, SynthLocal::write_into)?;
        self.func_expr.write_into(&mut buf)?;
        log::trace!("code size to write: {}", buf.len());

        wr.write_u32(
            buf.len()
                .try_into()
                .expect("function expression size overflow"),
        )?;
        wr.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SynthLocal {
    n: u32,
    t: ValueType,
}

impl SynthLocal {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_u32(self.n)?;
        self.t.write_into(wr)?;
        Ok(())
    }
}
