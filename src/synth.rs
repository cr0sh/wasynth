//! WebAssembly module 'synthesizer'.
//!
//! All types matching [`crate::parse`] should have `Synth` prefixes in its name.

pub mod sections;

/// A WebAssembly module synthesizer.
pub struct SynthModule {}
