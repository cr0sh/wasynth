use std::fmt::Debug;

use crate::{
    synth::sections::{SynthIndirectNameAssoc, SynthNameAssoc, SynthNameSection},
    Bytes, Error,
};

#[derive(Clone, Copy)]
pub struct NameSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> NameSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        let (name, bytes) = bytes.advance_name()?;
        assert_eq!(name, "name");
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> Result<SynthNameSection, Error> {
        trait IteratorExt: Iterator {
            fn extract_element(
                self,
                section_name: &'static str,
            ) -> Result<Option<Self::Item>, Error>;
        }

        impl<T, I: Iterator<Item = T>> IteratorExt for I {
            fn extract_element(mut self, section_name: &'static str) -> Result<Option<T>, Error> {
                let first = self.next();
                if self.next().is_some() {
                    return Err(Error::DuplicateNameSectionSubsection(section_name));
                }
                Ok(first)
            }
        }

        let sections = self.subsections()?.collect::<Vec<_>>();
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
            .map(|x| {
                x.name_assocs()
                    .unwrap()
                    .map(|x| x.map(NameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let local_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::LocalNames(_)))
            .extract_element("function names")?
            .map(|x| {
                x.indirect_name_assocs()
                    .unwrap()
                    .map(|x| x.map(IndirectNameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let label_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::LabelNames(_)))
            .extract_element("function names")?
            .map(|x| {
                x.indirect_name_assocs()
                    .unwrap()
                    .map(|x| x.map(IndirectNameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let type_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::TypeNames(_)))
            .extract_element("function names")?
            .map(|x| {
                x.name_assocs()
                    .unwrap()
                    .map(|x| x.map(NameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let table_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::TableNames(_)))
            .extract_element("function names")?
            .map(|x| {
                x.name_assocs()
                    .unwrap()
                    .map(|x| x.map(NameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let memory_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::MemoryNames(_)))
            .extract_element("function names")?
            .map(|x| {
                x.name_assocs()
                    .unwrap()
                    .map(|x| x.map(NameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let global_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::GlobalNames(_)))
            .extract_element("function names")?
            .map(|x| {
                x.name_assocs()
                    .unwrap()
                    .map(|x| x.map(NameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let element_segment_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::ElementSegmentNames(_)))
            .extract_element("function names")?
            .map(|x| {
                x.name_assocs()
                    .unwrap()
                    .map(|x| x.map(NameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let data_segment_names = sections
            .iter()
            .filter(|x| matches!(x, NameSubsection::DataSegmentNames(_)))
            .extract_element("function names")?
            .map(|x| {
                x.name_assocs()
                    .unwrap()
                    .map(|x| x.map(NameAssoc::into_synth))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(SynthNameSection {
            module_name,
            function_names,
            local_names,
            label_names,
            type_names,
            table_names,
            memory_names,
            global_names,
            element_segment_names,
            data_segment_names,
        })
    }

    pub fn subsections(&self) -> Result<impl Iterator<Item = NameSubsection<'bytes>> + '_, Error> {
        let mut subsections = Vec::new();
        let mut bytes = self.bytes;
        while !bytes.is_empty() {
            let (s, bytes_) = NameSubsection::from_bytes(bytes)?;
            bytes = bytes_;
            subsections.push(s);
        }

        Ok(subsections.into_iter())
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
    LabelNames(&'bytes [u8]),
    TypeNames(&'bytes [u8]),
    TableNames(&'bytes [u8]),
    MemoryNames(&'bytes [u8]),
    GlobalNames(&'bytes [u8]),
    ElementSegmentNames(&'bytes [u8]),
    DataSegmentNames(&'bytes [u8]),
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
                let (name, _bytes) = bytes.advance_name()?;
                Ok((Self::ModuleName(name), rest))
            }
            1 => Ok((Self::FunctionNames(bytes), rest)),
            2 => Ok((Self::LocalNames(bytes), rest)),
            3 => Ok((Self::LabelNames(bytes), rest)),
            4 => Ok((Self::TypeNames(bytes), rest)),
            5 => Ok((Self::TableNames(bytes), rest)),
            6 => Ok((Self::MemoryNames(bytes), rest)),
            7 => Ok((Self::GlobalNames(bytes), rest)),
            8 => Ok((Self::ElementSegmentNames(bytes), rest)),
            9 => Ok((Self::DataSegmentNames(bytes), rest)),
            other => Err(Error::NameSectionSubsectionId(other)),
        }
    }

    pub(crate) fn name_assocs(
        &self,
    ) -> Result<impl Iterator<Item = Result<NameAssoc<'bytes>, Error>> + '_, Error> {
        match self {
            NameSubsection::FunctionNames(x)
            | NameSubsection::TypeNames(x)
            | NameSubsection::TableNames(x)
            | NameSubsection::MemoryNames(x)
            | NameSubsection::GlobalNames(x)
            | NameSubsection::ElementSegmentNames(x)
            | NameSubsection::DataSegmentNames(x) => x.advance_vector(NameAssoc::from_bytes),
            _ => Err(Error::IncorrectSubsection),
        }
    }

    pub(crate) fn indirect_name_assocs(
        &self,
    ) -> Result<impl Iterator<Item = Result<IndirectNameAssoc<'bytes>, Error>> + '_, Error> {
        match self {
            NameSubsection::LocalNames(x) | NameSubsection::LabelNames(x) => {
                x.advance_vector(IndirectNameAssoc::from_bytes)
            }
            _ => Err(Error::IncorrectSubsection),
        }
    }
}

pub struct NameAssoc<'bytes> {
    pub(crate) idx: u32,
    pub(crate) name: &'bytes str,
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
    pub(crate) idx: u32,
    pub(crate) name_map: Vec<NameAssoc<'bytes>>,
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
