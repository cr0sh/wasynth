#[derive(Clone, Debug)]
pub struct SynthNameSection {
    pub(crate) module_name: String,
    pub(crate) function_names: Vec<SynthNameAssoc>,
    pub(crate) local_names: Vec<SynthIndirectNameAssoc>,
}

impl SynthNameSection {
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
}

#[derive(Clone, Debug)]
pub struct SynthNameAssoc {
    pub(crate) idx: u32,
    pub(crate) name: String,
}

#[derive(Clone, Debug)]
pub struct SynthIndirectNameAssoc {
    pub(crate) idx: u32,
    pub(crate) name_map: Vec<SynthNameAssoc>,
}
