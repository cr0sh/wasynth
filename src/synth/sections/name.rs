use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug)]
pub struct SynthNameSection {
    pub(crate) module_name: String,
    pub(crate) function_names: Vec<SynthNameAssoc>,
    pub(crate) local_names: Vec<SynthIndirectNameAssoc>,
    pub(crate) label_names: Vec<SynthIndirectNameAssoc>,
    pub(crate) type_names: Vec<SynthNameAssoc>,
    pub(crate) table_names: Vec<SynthNameAssoc>,
    pub(crate) memory_names: Vec<SynthNameAssoc>,
    pub(crate) global_names: Vec<SynthNameAssoc>,
    pub(crate) element_segment_names: Vec<SynthNameAssoc>,
    pub(crate) data_segment_names: Vec<SynthNameAssoc>,
}

impl SynthNameSection {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        fn write_subsection(
            subsection_id: u8,
            wr: &mut impl Write,
            func: impl Fn(&mut Vec<u8>) -> Result<(), io::Error>,
        ) -> Result<(), io::Error> {
            let mut buf = Vec::new();
            func(&mut buf)?;
            wr.write_all(&[subsection_id])?;
            wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
            wr.write_all(&buf)?;
            Ok(())
        }

        write_subsection(0, wr, |wr| wr.write_name(&self.module_name))?;
        write_subsection(1, wr, |wr| {
            wr.write_vector(&self.function_names, SynthNameAssoc::write_into)
        })?;
        write_subsection(2, wr, |wr| {
            wr.write_vector(&self.local_names, SynthIndirectNameAssoc::write_into)
        })?;

        Ok(())
    }

    pub fn module_name(&self) -> &str {
        self.module_name.as_ref()
    }

    pub fn module_name_mut(&mut self) -> &mut String {
        &mut self.module_name
    }

    pub fn function_names(&self) -> &[SynthNameAssoc] {
        self.function_names.as_ref()
    }

    pub fn function_names_mut(&mut self) -> &mut Vec<SynthNameAssoc> {
        &mut self.function_names
    }

    pub fn local_names(&self) -> &[SynthIndirectNameAssoc] {
        self.local_names.as_ref()
    }

    pub fn local_names_mut(&mut self) -> &mut Vec<SynthIndirectNameAssoc> {
        &mut self.local_names
    }

    pub fn label_names(&self) -> &[SynthIndirectNameAssoc] {
        self.label_names.as_ref()
    }

    pub fn label_names_mut(&mut self) -> &mut Vec<SynthIndirectNameAssoc> {
        &mut self.label_names
    }

    pub fn type_names(&self) -> &[SynthNameAssoc] {
        self.type_names.as_ref()
    }

    pub fn type_names_mut(&mut self) -> &mut Vec<SynthNameAssoc> {
        &mut self.type_names
    }

    pub fn table_names(&self) -> &[SynthNameAssoc] {
        self.table_names.as_ref()
    }

    pub fn table_names_mut(&mut self) -> &mut Vec<SynthNameAssoc> {
        &mut self.table_names
    }

    pub fn memory_names(&self) -> &[SynthNameAssoc] {
        self.memory_names.as_ref()
    }

    pub fn memory_names_mut(&mut self) -> &mut Vec<SynthNameAssoc> {
        &mut self.memory_names
    }

    pub fn global_names(&self) -> &[SynthNameAssoc] {
        self.global_names.as_ref()
    }

    pub fn global_names_mut(&mut self) -> &mut Vec<SynthNameAssoc> {
        &mut self.global_names
    }

    pub fn element_segment_names(&self) -> &[SynthNameAssoc] {
        self.element_segment_names.as_ref()
    }

    pub fn element_segment_names_mut(&mut self) -> &mut Vec<SynthNameAssoc> {
        &mut self.element_segment_names
    }

    pub fn data_segment_names(&self) -> &[SynthNameAssoc] {
        self.data_segment_names.as_ref()
    }

    pub fn data_segment_names_mut(&mut self) -> &mut Vec<SynthNameAssoc> {
        &mut self.data_segment_names
    }
}

#[derive(Clone, Debug)]
pub struct SynthNameAssoc {
    pub(crate) idx: u32,
    pub(crate) name: String,
}

impl SynthNameAssoc {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_u32(self.idx)?;
        wr.write_name(&self.name)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SynthIndirectNameAssoc {
    pub(crate) idx: u32,
    pub(crate) name_map: Vec<SynthNameAssoc>,
}

impl SynthIndirectNameAssoc {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_u32(self.idx)?;
        wr.write_vector(&self.name_map, SynthNameAssoc::write_into)?;

        Ok(())
    }
}
