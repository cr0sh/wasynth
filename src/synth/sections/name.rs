use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug)]
pub struct SynthNameSection {
    pub(crate) module_name: Option<String>,
    pub(crate) function_names: Option<Vec<SynthNameAssoc>>,
    pub(crate) local_names: Option<Vec<SynthIndirectNameAssoc>>,
    pub(crate) label_names: Option<Vec<SynthIndirectNameAssoc>>,
    pub(crate) type_names: Option<Vec<SynthNameAssoc>>,
    pub(crate) table_names: Option<Vec<SynthNameAssoc>>,
    pub(crate) memory_names: Option<Vec<SynthNameAssoc>>,
    pub(crate) global_names: Option<Vec<SynthNameAssoc>>,
    pub(crate) element_segment_names: Option<Vec<SynthNameAssoc>>,
    pub(crate) data_segment_names: Option<Vec<SynthNameAssoc>>,
}

impl SynthNameSection {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_name("name")?;

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

        if let Some(name) = self.module_name.as_ref() {
            write_subsection(0, &mut buf, |wr| wr.write_name(name))?;
        }

        if let Some(assocs) = self.function_names.as_ref() {
            write_subsection(1, &mut buf, |wr| {
                wr.write_vector(assocs, SynthNameAssoc::write_into)
            })?;
        }

        if let Some(indirect_assocs) = self.local_names.as_ref() {
            write_subsection(2, &mut buf, |wr| {
                wr.write_vector(indirect_assocs, SynthIndirectNameAssoc::write_into)
            })?;
        }

        if let Some(indirect_assocs) = self.label_names.as_ref() {
            write_subsection(3, &mut buf, |wr| {
                wr.write_vector(indirect_assocs, SynthIndirectNameAssoc::write_into)
            })?;
        }

        if let Some(assocs) = self.type_names.as_ref() {
            write_subsection(4, &mut buf, |wr| {
                wr.write_vector(assocs, SynthNameAssoc::write_into)
            })?;
        }

        if let Some(assocs) = self.table_names.as_ref() {
            write_subsection(5, &mut buf, |wr| {
                wr.write_vector(assocs, SynthNameAssoc::write_into)
            })?;
        }

        if let Some(assocs) = self.memory_names.as_ref() {
            write_subsection(6, &mut buf, |wr| {
                wr.write_vector(assocs, SynthNameAssoc::write_into)
            })?;
        }

        if let Some(assocs) = self.global_names.as_ref() {
            write_subsection(7, &mut buf, |wr| {
                wr.write_vector(assocs, SynthNameAssoc::write_into)
            })?;
        }

        if let Some(assocs) = self.element_segment_names.as_ref() {
            write_subsection(8, &mut buf, |wr| {
                wr.write_vector(assocs, SynthNameAssoc::write_into)
            })?;
        }

        if let Some(assocs) = self.data_segment_names.as_ref() {
            write_subsection(9, &mut buf, |wr| {
                wr.write_vector(assocs, SynthNameAssoc::write_into)
            })?;
        }

        wr.write_all(&[0])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }

    pub fn module_name(&self) -> Option<&str> {
        self.module_name.as_deref()
    }

    pub fn module_name_mut(&mut self) -> Option<&mut String> {
        self.module_name.as_mut()
    }

    pub fn function_names(&self) -> Option<&[SynthNameAssoc]> {
        self.function_names.as_deref()
    }

    pub fn function_names_mut(&mut self) -> &mut Option<Vec<SynthNameAssoc>> {
        &mut self.function_names
    }

    pub fn local_names(&self) -> Option<&[SynthIndirectNameAssoc]> {
        self.local_names.as_deref()
    }

    pub fn local_names_mut(&mut self) -> &mut Option<Vec<SynthIndirectNameAssoc>> {
        &mut self.local_names
    }

    pub fn label_names(&self) -> Option<&[SynthIndirectNameAssoc]> {
        self.label_names.as_deref()
    }

    pub fn label_names_mut(&mut self) -> &mut Option<Vec<SynthIndirectNameAssoc>> {
        &mut self.label_names
    }

    pub fn type_names(&self) -> Option<&[SynthNameAssoc]> {
        self.type_names.as_deref()
    }

    pub fn type_names_mut(&mut self) -> &mut Option<Vec<SynthNameAssoc>> {
        &mut self.type_names
    }

    pub fn table_names(&self) -> Option<&[SynthNameAssoc]> {
        self.table_names.as_deref()
    }

    pub fn table_names_mut(&mut self) -> &mut Option<Vec<SynthNameAssoc>> {
        &mut self.table_names
    }

    pub fn memory_names(&self) -> Option<&[SynthNameAssoc]> {
        self.memory_names.as_deref()
    }

    pub fn memory_names_mut(&mut self) -> &mut Option<Vec<SynthNameAssoc>> {
        &mut self.memory_names
    }

    pub fn global_names(&self) -> Option<&[SynthNameAssoc]> {
        self.global_names.as_deref()
    }

    pub fn global_names_mut(&mut self) -> &mut Option<Vec<SynthNameAssoc>> {
        &mut self.global_names
    }

    pub fn element_segment_names(&self) -> Option<&[SynthNameAssoc]> {
        self.element_segment_names.as_deref()
    }

    pub fn element_segment_names_mut(&mut self) -> &mut Option<Vec<SynthNameAssoc>> {
        &mut self.element_segment_names
    }

    pub fn data_segment_names(&self) -> Option<&[SynthNameAssoc]> {
        self.data_segment_names.as_deref()
    }

    pub fn data_segment_names_mut(&mut self) -> &mut Option<Vec<SynthNameAssoc>> {
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
