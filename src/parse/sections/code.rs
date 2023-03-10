use std::fmt::Debug;

use crate::{
    instructions::Expression,
    synth::sections::{SynthCode, SynthCodeSection},
    wasm_types::ValueType,
    Bytes, Error,
};

#[derive(Clone, Copy)]
pub struct CodeSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> CodeSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> Result<SynthCodeSection, Error> {
        Ok(SynthCodeSection {
            codes: self
                .codes()?
                .map(|x| x.map(Code::into_synth))
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }

    pub fn codes(&self) -> Result<impl Iterator<Item = Result<Code, Error>> + '_, Error> {
        self.bytes.advance_vector(Code::from_bytes)
    }
}

impl<'bytes> Debug for CodeSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CodeSection").finish()
    }
}

#[derive(Clone, Debug)]
pub struct Code {
    locals: Vec<Local>,
    func_expr: Expression,
}

impl Code {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (size, bytes) = bytes.advance_u32()?;
        let size_u = usize::try_from(size).expect("code size overflow");
        let code_bytes = &bytes[..size_u];
        log::trace!("code size = {size}, reading locals");
        let mut localit = code_bytes.advance_vector(Local::from_bytes)?;
        let mut locals = Vec::new();
        for local in &mut localit {
            locals.push(local?);
        }
        let code_bytes = localit.finalize();
        log::trace!("reading func_expr");
        let (func_expr, code_bytes) = Expression::from_bytes(code_bytes)?;

        if !code_bytes.is_empty() {
            return Err(Error::TrailingBytes);
        }

        Ok((Self { locals, func_expr }, &bytes[size_u..]))
    }

    pub(crate) fn into_synth(self) -> SynthCode {
        let mut locals = Vec::new();
        for Local { n, t } in self.locals {
            for _i in 0..n {
                locals.push(t);
            }
        }
        SynthCode {
            locals,
            func_expr: self.func_expr,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Local {
    n: u32,
    t: ValueType,
}

impl Local {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (n, bytes) = bytes.advance_u32()?;
        let (&[t], bytes) = bytes.advance()?;
        let t = ValueType::from_byte(t)?;
        Ok((Self { n, t }, bytes))
    }
}
