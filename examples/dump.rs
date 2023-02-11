fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let wasm = wat::parse_str(include_str!("../tests/cases/fib_log.wat"))?;
    let module = wasynth::parse::Module::from_binary(&wasm)?;

    for section in module.sections() {
        match section {
            wasynth::parse::Section::Custom(x) => {
                println!(
                    "custom: name={}, payload={:?}, payload_str_lossy={}",
                    x.name(),
                    x.bytes(),
                    String::from_utf8_lossy(x.bytes())
                );
            }
            wasynth::parse::Section::Type(tysec) => {
                println!("types:");
                for ty in tysec.types()? {
                    println!("{}", ty?);
                }
            }
            wasynth::parse::Section::Import(imsec) => {
                println!("imports:");
                for im in imsec.imports()? {
                    println!("{:?}", im?);
                }
            }
            wasynth::parse::Section::Function(funcsec) => {
                println!("function type indices:");
                for tyidx in funcsec.type_indices()? {
                    println!("{}", tyidx?);
                }
            }
            wasynth::parse::Section::Table(tablesec) => {
                println!("tables:");
                for table in tablesec.tables()? {
                    println!("{:?}", table?);
                }
            }
            wasynth::parse::Section::Memory(memsec) => {
                println!("memories:");
                for mem in memsec.memories()? {
                    println!("{:?}", mem?);
                }
            }
            wasynth::parse::Section::Global(_) => (),
            wasynth::parse::Section::Export(_) => (),
            wasynth::parse::Section::Start(_) => (),
            wasynth::parse::Section::Element(_) => (),
            wasynth::parse::Section::Code(codesec) => {
                println!("codes:");
                for code in codesec.codes()? {
                    println!("{:?}", code?);
                }
            }
            wasynth::parse::Section::Data(datasec) => {
                println!("data:");
                for data in datasec.all_data()? {
                    println!("{:?}", data?);
                }
            }
            wasynth::parse::Section::DataCount(datacountsec) => {
                println!("data count: {}", datacountsec.data_count())
            }
        }
    }

    Ok(())
}
