//! WebAssembly module 'synthesizer'.
//!
//! All types matching [`crate::parse`] should have `Synth` prefixes in its name.

use std::io::{self, Write};

use crate::{WASM_MAGIC, WASM_VERSION};

use self::sections::{
    SynthCodeSection, SynthCustomSection, SynthDataCountSection, SynthDataSection,
    SynthElementSection, SynthExportSection, SynthFunctionSection, SynthGlobalSection,
    SynthImportSection, SynthMemorySection, SynthNameSection, SynthStartSection, SynthTableSection,
    SynthTypeSection,
};

pub mod sections;

/// A WebAssembly module synthesizer.
pub struct SynthModule {
    pub(crate) type_section: Option<SynthTypeSection>,
    pub(crate) import_section: Option<SynthImportSection>,
    pub(crate) function_section: Option<SynthFunctionSection>,
    pub(crate) table_section: Option<SynthTableSection>,
    pub(crate) memory_section: Option<SynthMemorySection>,
    pub(crate) global_section: Option<SynthGlobalSection>,
    pub(crate) export_section: Option<SynthExportSection>,
    pub(crate) start_section: Option<SynthStartSection>,
    pub(crate) element_section: Option<SynthElementSection>,
    pub(crate) code_section: Option<SynthCodeSection>,
    pub(crate) data_section: Option<SynthDataSection>,
    pub(crate) data_count_section: Option<SynthDataCountSection>,
    pub(crate) custom_sections: Vec<SynthCustomSection>,
    pub(crate) name_section: Option<SynthNameSection>,
}

impl SynthModule {
    pub fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_all(WASM_MAGIC)?;
        wr.write_all(&WASM_VERSION.to_le_bytes())?;

        if let Some(sec) = &self.type_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.import_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.function_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.table_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.memory_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.global_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.export_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.start_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.element_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.code_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.data_section {
            sec.write_into(wr)?;
        }
        if let Some(sec) = &self.data_count_section {
            sec.write_into(wr)?;
        }

        for section in &self.custom_sections {
            section.write_into(&mut wr)?;
        }

        Ok(())
    }
}
