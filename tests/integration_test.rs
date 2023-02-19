use std::sync::Once;

use wasynth::{
    parse::{
        sections::{
            CodeSection, DataSection, FunctionSection, ImportSection, MemorySection, TableSection,
            TypeSection,
        },
        Module, Section,
    },
    Error,
};

fn init_logger() {
    static ONCE: Once = Once::new();
    ONCE.call_once(env_logger::init);
}

fn parse_wat(wat_s: &str) -> Module {
    let wasm = wat::parse_str(wat_s).expect("cannot parse wat");

    // NOTE: wasm binary contents are leaked here
    parse_wasm(wasm.leak())
}

fn parse_wasm(wasm: &[u8]) -> Module {
    wasmparser::validate(&wasm).expect("pre-parse validation fail");
    wasynth::parse::Module::from_binary(wasm).expect("cannot parse wasm")
}

fn type_section(section: &TypeSection) -> Result<(), Error> {
    for ty in section.types()? {
        ty?;
    }
    Ok(())
}

fn import_section(section: &ImportSection) -> Result<(), Error> {
    for im in section.imports()? {
        im?;
    }
    Ok(())
}

fn function_section(section: &FunctionSection) -> Result<(), Error> {
    for tyidx in section.type_indices()? {
        tyidx?;
    }
    Ok(())
}

fn table_section(section: &TableSection) -> Result<(), Error> {
    for table in section.tables()? {
        table?;
    }
    Ok(())
}

fn memory_section(section: &MemorySection) -> Result<(), Error> {
    for mem in section.memories()? {
        mem?;
    }
    Ok(())
}

fn code_section(section: &CodeSection) -> Result<(), Error> {
    for code in section.codes()? {
        code?;
    }
    Ok(())
}

fn data_section(section: &DataSection) -> Result<(), Error> {
    for data in section.all_data()? {
        data?;
    }
    Ok(())
}

fn test_sections(module: &Module) {
    for section in module.sections() {
        match section {
            Section::Custom(_) => (),
            Section::Type(s) => type_section(s).expect("cannot parse type section"),
            Section::Import(s) => import_section(s).expect("cannot parse import section"),
            Section::Function(s) => function_section(s).expect("cannot parse function section"),
            Section::Table(s) => table_section(s).expect("cannot parse table section"),
            Section::Memory(s) => memory_section(s).expect("cannot parse memory section"),
            Section::Global(_) => (),
            Section::Export(_) => (),
            Section::Start(_) => (),
            Section::Element(_) => (),
            Section::Code(s) => code_section(s).expect("cannot parse code section"),
            Section::Data(s) => data_section(s).expect("cannot parse data section"),
            Section::DataCount(_) => (),
        }
    }
}

fn test_synth(module: &Module) {
    let mut buf = Vec::new();
    module
        .clone()
        .into_synth()
        .expect("into_synth fail")
        .write_into(&mut buf)
        .expect("write_into fail");

    wasmparser::validate(&buf).expect("validation fail");
}

mod autogenerated_from_files {
    use super::*;

    tests_gen::generate_tests!();
}
