#![no_main]

use libfuzzer_sys::fuzz_target;
use wasm_smith::Module;

fuzz_target!(|module: Module| {
    let wasm_bytes = module.to_bytes();

    let module = wasynth::parse::Module::from_binary(&wasm_bytes).expect("cannot parse module");

    module.validate().expect("validation failed");
});
