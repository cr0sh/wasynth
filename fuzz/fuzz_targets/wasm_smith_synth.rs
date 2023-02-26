#![no_main]

use std::sync::Once;

use libfuzzer_sys::fuzz_target;
use wasm_smith::Module;

static INIT_ONCE: Once = Once::new();

fuzz_target!(|module: Module| {
    INIT_ONCE.call_once(env_logger::init);

    let wasm_bytes = module.to_bytes();

    let module = wasynth::parse::Module::from_binary(&wasm_bytes).expect("cannot parse module");
    let mut wasm_bytes = Vec::new();
    module
        .into_synth()
        .expect("into_synth failed")
        .write_into(&mut wasm_bytes)
        .expect("write_into failed");

    wasynth::parse::Module::from_binary(&wasm_bytes).expect("cannot parse synthesized module");
    wasmparser::validate(&wasm_bytes).expect("wasmparser validation failed");
});
