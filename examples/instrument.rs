use difference::Changeset;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let s = include_bytes!("../tests/cases/wasynth_release.wasm");
    let wasm = wat::parse_bytes(s)?;
    let before = wasmprinter::print_bytes(wasm.clone()).unwrap();
    let mut module = wasm_instrument::parse::Module::from_binary(&wasm)?.into_synth()?;
    wasm_instrument::instrument::install_all(&mut module)?;

    let mut buf = Vec::new();
    module.write_into(&mut buf)?;

    let after = wasmprinter::print_bytes(&buf)?;

    let diff = Changeset::new(&before, &after, "\n");
    eprintln!("{diff}");

    Ok(())
}
