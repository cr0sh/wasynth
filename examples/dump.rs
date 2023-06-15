fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let wasm = wat::parse_str(include_str!("../tests/cases/fib_log.wat"))?;
    let module = wasm_instrument::parse::Module::from_binary(&wasm)?;

    for section in module.sections() {
        match section {
            wasm_instrument::parse::Section::Custom(x) => {
                println!(
                    "custom: name={}, payload={:?}, payload_str_lossy={}",
                    x.name(),
                    x.bytes(),
                    String::from_utf8_lossy(x.bytes())
                );
            }
            wasm_instrument::parse::Section::Type(tysec) => {
                println!("types:");
                for ty in tysec.types()? {
                    println!("{}", ty?);
                }
            }
            wasm_instrument::parse::Section::Import(imsec) => {
                println!("imports:");
                for im in imsec.imports()? {
                    println!("{:?}", im?);
                }
            }
            wasm_instrument::parse::Section::Function(funcsec) => {
                println!("function type indices:");
                for tyidx in funcsec.type_indices()? {
                    println!("{}", tyidx?);
                }
            }
            wasm_instrument::parse::Section::Table(tablesec) => {
                println!("tables:");
                for table in tablesec.tables()? {
                    println!("{:?}", table?);
                }
            }
            wasm_instrument::parse::Section::Memory(memsec) => {
                println!("memories:");
                for mem in memsec.memories()? {
                    println!("{:?}", mem?);
                }
            }
            wasm_instrument::parse::Section::Global(_) => (),
            wasm_instrument::parse::Section::Export(_) => (),
            wasm_instrument::parse::Section::Start(_) => (),
            wasm_instrument::parse::Section::Element(_) => (),
            wasm_instrument::parse::Section::Code(codesec) => {
                println!("codes:");
                for code in codesec.codes()? {
                    println!("{:?}", code?);
                }
            }
            wasm_instrument::parse::Section::Data(datasec) => {
                println!("data:");
                for data in datasec.all_data()? {
                    println!("{:?}", data?);
                }
            }
            wasm_instrument::parse::Section::DataCount(datacountsec) => {
                println!("data count: {}", datacountsec.data_count())
            }
            wasm_instrument::parse::Section::Name(namesec) => {
                println!("{:?}", namesec)
            }
        }
    }

    Ok(())
}
