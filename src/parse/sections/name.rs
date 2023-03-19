use std::fmt::Debug;

use crate::{
    synth::sections::{SynthIndirectNameAssoc, SynthNameAssoc, SynthNameSection},
    Bytes, Error, VectorIterator,
};

#[derive(Clone, Copy)]
pub struct NameSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> NameSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> Result<SynthNameSection, Error> {
        trait IteratorExt: Iterator {
            fn extract_element(self, section_name: &'static str) -> Result<Self::Item, Error>;
        }

        impl<T, I: Iterator<Item = T>> IteratorExt for I {
            fn extract_element(mut self, section_name: &'static str) -> Result<T, Error> {
                let first = self
                    .next()
                    .ok_or(Error::MissingNameSectionSubsection(section_name))?;
                if self.next().is_some() {
                    return Err(Error::DuplicateNameSectionSubsection(section_name));
                }
                Ok(first)
            }
        }

        let sections = self.subsections()?.collect::<Result<Vec<_>, _>>()?;
        let module_name = sections
            .iter()
            .filter_map(|x| match x {
                NameSubsection::ModuleName(x) => Some(x.to_string()),
                _ => None,
            })
            .extract_element("module name")?;

        let function_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::FunctionNames(_)))
            .extract_element("function names")?
            .name_assocs()?
            .map(|x| x.map(NameAssoc::into_synth))
            .collect::<Result<Vec<_>, _>>()?;

        let local_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::LocalNames(_)))
            .extract_element("local names")?
            .indirect_name_assocs()?
            .map(|x| x.map(IndirectNameAssoc::into_synth))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SynthNameSection {
            module_name,
            function_names,
            local_names,
        })
    }

    pub fn subsections(
        &self,
    ) -> Result<impl Iterator<Item = Result<NameSubsection<'bytes>, Error>> + '_, Error> {
        self.bytes.advance_vector(NameSubsection::from_bytes)
    }
}

impl<'bytes> Debug for NameSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NameSection").finish()
    }
}

#[derive(Clone, Copy)]
pub enum NameSubsection<'bytes> {
    ModuleName(&'bytes str),
    FunctionNames(&'bytes [u8]),
    LocalNames(&'bytes [u8]),
}

impl<'bytes> NameSubsection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<(Self, &'bytes [u8]), Error> {
        let (&[id], bytes) = bytes.advance()?;
        let (size, bytes) = bytes.advance_u32()?;
        let size = size.try_into().expect("subsection size overflow");
        let rest = &bytes[size..];
        let bytes = &bytes[..size];
        match id {
            0 => {
                let (name, bytes) = bytes.advance_name()?;
                Ok((Self::ModuleName(name), rest))
            }
            1 => Ok((Self::FunctionNames(bytes), rest)),
            2 => Ok((Self::LocalNames(bytes), rest)),
            other => Err(Error::NameSectionSubsectionId(other)),
        }
    }

    pub(crate) fn name_assocs(
        &self,
    ) -> Result<impl Iterator<Item = Result<NameAssoc<'bytes>, Error>> + '_, Error> {
        match self {
            NameSubsection::FunctionNames(x) => x.advance_vector(NameAssoc::from_bytes),
            _ => return Err(Error::IncorrectSubsection),
        }
    }

    pub(crate) fn indirect_name_assocs(
        &self,
    ) -> Result<impl Iterator<Item = Result<IndirectNameAssoc<'bytes>, Error>> + '_, Error> {
        match self {
            NameSubsection::LocalNames(x) => x.advance_vector(IndirectNameAssoc::from_bytes),
            _ => return Err(Error::IncorrectSubsection),
        }
    }
}

pub struct NameAssoc<'bytes> {
    idx: u32,
    name: &'bytes str,
}

impl<'bytes> NameAssoc<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<(Self, &'bytes [u8]), Error> {
        let (idx, bytes) = bytes.advance_u32()?;
        let (name, bytes) = bytes.advance_name()?;
        Ok((Self { idx, name }, bytes))
    }

    pub(crate) fn into_synth(self) -> SynthNameAssoc {
        SynthNameAssoc {
            idx: self.idx,
            name: self.name.to_string(),
        }
    }
}

pub struct IndirectNameAssoc<'bytes> {
    idx: u32,
    name_map: Vec<NameAssoc<'bytes>>,
}

impl<'bytes> IndirectNameAssoc<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<(Self, &'bytes [u8]), Error> {
        let (idx, bytes) = bytes.advance_u32()?;
        let mut name_map = Vec::new();

        let mut it = bytes.advance_vector(NameAssoc::from_bytes)?;
        for na in &mut it {
            name_map.push(na?);
        }
        let bytes = it.finalize();

        Ok((IndirectNameAssoc { idx, name_map }, bytes))
    }

    pub(crate) fn into_synth(self) -> SynthIndirectNameAssoc {
        SynthIndirectNameAssoc {
            idx: self.idx,
            name_map: self
                .name_map
                .into_iter()
                .map(NameAssoc::into_synth)
                .collect(),
        }
    }
}
