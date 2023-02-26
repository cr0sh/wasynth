#![no_main]

use std::sync::Once;

use libfuzzer_sys::fuzz_target;
use wasm_smith::Module;

static INIT_ONCE: Once = Once::new();

fuzz_target!(|module: Module| {
    INIT_ONCE.call_once(env_logger::init);

    let wasm_bytes = module.to_bytes();

    let module = wasynth::parse::Module::from_binary(&wasm_bytes).expect("cannot parse module");

    module.validate().expect("validation failed");
});
