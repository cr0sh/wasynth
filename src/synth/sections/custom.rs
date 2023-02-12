#[derive(Clone, Debug)]
pub struct CustomSection {
    pub(in crate::synth) name: String,
    pub(in crate::synth) bytes: Vec<u8>,
}

impl CustomSection {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
}
