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
        wr.write_vector(&self.codes, SynthCode::write_into)
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
        if let Some(mut ty) = self.locals.first().copied() {
            let mut cnt = 0;
            for local in &self.locals {
                if ty == *local {
                    cnt += 1;
                } else {
                    buf.write_u32(cnt)?;
                    ty.write_into(&mut buf)?;
                    ty = *local;
                    cnt = 1;
                }
            }
            buf.write_u32(cnt)?;
            ty.write_into(&mut buf)?;
        }

        self.func_expr.write_into(&mut buf)?;

        wr.write_u32(
            buf.len()
                .try_into()
                .expect("function expression size overflow"),
        )?;
        wr.write_all(&buf)?;

        Ok(())
    }
}
