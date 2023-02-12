#[derive(Clone, Debug)]
pub struct ExportSection {
    pub(in crate::synth) bytes: Vec<u8>,
}
