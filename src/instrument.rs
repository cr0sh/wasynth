use crate::{
    instructions::{Expression, Instruction},
    synth::{
        sections::{
            SynthCode, SynthData, SynthElemInit::FuncIndices, SynthExportDescription, SynthImport,
            SynthImportDescription, SynthNameAssoc,
        },
        SynthModule,
    },
    wasm_types::{FuncType, ResultType, ValueType},
    Error,
};

fn trampoline_instrs(target_funcidx: usize, enter_funcidx: u32, leave_funcidx: u32) -> Expression {
    let target_funcidx = target_funcidx.try_into().expect("function index overflow");
    Expression(vec![
        Instruction::I32Const(target_funcidx),
        Instruction::Call(enter_funcidx),
        Instruction::Call(target_funcidx.try_into().expect("function index overflow")),
        Instruction::I32Const(target_funcidx),
        Instruction::Call(leave_funcidx),
    ])
}

/// Installs instrumentation hook for every function on the module.
pub fn install_all(module: &mut SynthModule) -> Result<(), Error> {
    // TODO: ensure idempotence (by checking a certain custom section then inserting it)

    let tysec = &mut module
        .type_section
        .get_or_insert_with(Default::default)
        .types;
    let import_tyidx = tysec.len();
    tysec.push(FuncType {
        param: ResultType(vec![ValueType::I32]),
        result: ResultType(vec![]),
    });

    let imports = &mut module
        .import_section
        .get_or_insert_with(Default::default)
        .imports;

    let enter_hook_funcidx: u32 = imports
        .iter()
        .filter(|x| matches!(x.description, SynthImportDescription::Type(..)))
        .count()
        .try_into()
        .expect("function index overflow");
    let leave_hook_funcidx = enter_hook_funcidx
        .checked_add(1)
        .expect("function index overflow");

    imports.push(SynthImport {
        module: String::from("wasynth_hooks"),
        name: String::from("enter"),
        description: SynthImportDescription::Type(
            import_tyidx.try_into().expect("type index overflow"),
        ),
    });
    imports.push(SynthImport {
        module: String::from("wasynth_hooks"),
        name: String::from("leave"),
        description: SynthImportDescription::Type(
            import_tyidx.try_into().expect("type index overflow"),
        ),
    });

    let type_indices = &module
        .function_section
        .get_or_insert_with(Default::default)
        .type_indices;

    let codesec = module.code_section.get_or_insert_with(Default::default);

    assert_eq!(type_indices.len(), codesec.codes().len());

    let mut funcs_to_append = Vec::new();
    let mut codes_to_append = Vec::new();
    for (funcidx, (tyidx, code)) in type_indices
        .iter()
        .copied()
        .zip(codesec.codes_mut().iter_mut())
        .enumerate()
        .map(|(funcidx, y)| (funcidx + imports.len() + type_indices.len(), y))
    {
        let original_instrs = std::mem::replace(
            &mut code.func_expr,
            trampoline_instrs(funcidx, enter_hook_funcidx, leave_hook_funcidx),
        );

        codes_to_append.push(SynthCode {
            locals: code.locals.clone(),
            func_expr: original_instrs,
        });
        funcs_to_append.push(tyidx);

        code.locals.truncate(0); // locals are not needed on trampoline
    }

    codesec.codes_mut().extend_from_slice(&codes_to_append);

    module
        .function_section
        .as_mut()
        .ok_or(Error::MissingSection("function"))?
        .type_indices_mut()
        .extend_from_slice(&funcs_to_append);

    // increment function indices greater than or equal to import_hook_funcidx
    let fnidx_offset: u32 = (2 + funcs_to_append.len())
        .try_into()
        .expect("function offset overflow");
    let increment_fnidx = |x: &mut u32| {
        if *x >= enter_hook_funcidx {
            *x += fnidx_offset;
        }
    };

    if let Some(glsec) = module.global_section.as_mut() {
        for global in glsec.globals_mut() {
            global.init.visit_func_indices(increment_fnidx);
        }
    }

    if let Some(exsec) = module.export_section.as_mut() {
        for export in exsec.exports_mut() {
            if let SynthExportDescription::Func(idx) = &mut export.desc {
                increment_fnidx(idx);
            }
        }
    }

    if let Some(stsec) = module.start_section.as_mut() {
        increment_fnidx(&mut stsec.start);
    }

    if let Some(elsec) = module.element_section.as_mut() {
        for el in elsec.elements_mut() {
            if let FuncIndices(x) = &mut el.init {
                for x in x {
                    increment_fnidx(x);
                }
            }
        }
    }

    if let Some(codesec) = module.code_section.as_mut() {
        for code in codesec.codes_mut() {
            code.func_expr_mut().visit_func_indices(increment_fnidx);
        }
    }

    if let Some(datasec) = module.data_section.as_mut() {
        for data in datasec.all_data_mut() {
            match data {
                SynthData::Active { offset, .. } => offset.visit_func_indices(increment_fnidx),
                SynthData::Passive(_) => (),
            }
        }
    }

    if let Some(namesec) = module.name_section.as_mut() {
        if let Some(assocs) = namesec.function_names_mut() {
            for assoc in &mut *assocs {
                increment_fnidx(&mut assoc.idx);
            }
            assocs.push(SynthNameAssoc {
                idx: enter_hook_funcidx,
                name: String::from("wasynth_hooks/enter"),
            });
            assocs.push(SynthNameAssoc {
                idx: leave_hook_funcidx,
                name: String::from("wasynth_hooks/leave"),
            });
        }

        if let Some(indassocs) = namesec.local_names_mut() {
            for indassoc in &mut *indassocs {
                increment_fnidx(&mut indassoc.idx);
            }
        }

        if let Some(indassocs) = namesec.label_names_mut() {
            for indassoc in &mut *indassocs {
                increment_fnidx(&mut indassoc.idx);
            }
        }
    }

    Ok(())
}
