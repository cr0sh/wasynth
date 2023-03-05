use std::io::{self, Write};

use crate::{instructions::Expression, wasm_types::GlobalType, WriteExt};

#[derive(Clone, Debug, Default)]
pub struct SynthGlobalSection {
    pub(crate) globals: Vec<SynthGlobal>,
}

impl SynthGlobalSection {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_vector(&self.globals, SynthGlobal::write_into)?;

        wr.write_all(&[6])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SynthGlobal {
    pub(crate) ty: GlobalType,
    pub(crate) init: Expression,
}

impl SynthGlobal {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        self.ty.write_into(wr)?;
        self.init.write_into(wr)?;
        Ok(())
    }
}
