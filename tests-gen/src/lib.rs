use std::{ffi::OsStr, fs::read_dir, path::PathBuf};

use anyhow::{anyhow, bail, ensure, Context};
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitStr};

#[proc_macro]
pub fn generate_tests(_input: TokenStream) -> TokenStream {
    match generate_tests_inner() {
        Ok(x) => x,
        Err(e) => {
            let s = format!("{e:#?}");
            let lit = LitStr::new(&s, Span::call_site().into());
            quote! {
                compile_error!(#lit)
            }
            .into()
        }
    }
}

fn generate_tests_inner() -> anyhow::Result<TokenStream> {
    let cases_dir = {
        let mut manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir.pop();
        manifest_dir.push("tests");
        manifest_dir.push("cases");
        manifest_dir
    };

    ensure!(cases_dir.is_dir(), "tests/cases is not a directory");

    let toks = read_dir(cases_dir)
        .context("cannot read the tests/cases directory")?
        .map(|f| {
            let f = f.context("cannot read a directory entry")?;
            let path = f.path();
            let path_lit = LitStr::new(
                path.to_str()
                    .ok_or_else(|| anyhow!("cannot convert file path to &str"))?,
                Span::call_site().into(),
            );
            let fn_name = Ident::new_raw(
                path.file_stem()
                    .ok_or_else(|| anyhow!("no stem"))?
                    .to_str()
                    .ok_or_else(|| anyhow!("invalid stem"))?,
                Span::call_site().into(),
            );

            let const_name = Ident::new_raw(
                &format!(
                    "{}_CONTENTS",
                    path.file_stem()
                        .ok_or_else(|| anyhow!("no stem"))?
                        .to_str()
                        .ok_or_else(|| anyhow!("invalid stem"))?
                        .to_uppercase(),
                ),
                Span::call_site().into(),
            );

            match path.extension().and_then(OsStr::to_str) {
                Some("wat") => Ok::<TokenStream, anyhow::Error>(
                    quote! {
                        pub(super) const #const_name: &str = include_str!(#path_lit);

                        #[test]
                        fn #fn_name() {
                            let module = parse_wat(#const_name);
                            test_sections(&module);
                        }
                    }
                    .into(),
                ),
                Some("wasm") => Ok::<TokenStream, anyhow::Error>(
                    quote! {
                        pub(super) const #const_name: &[u8] = include_bytes!(#path_lit);

                        #[test]
                        fn #fn_name() {
                            let module = parse_wasm(#const_name);
                            test_sections(&module);
                        }
                    }
                    .into(),
                ),
                other => bail!("unknown extension {other:?}"),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut st = TokenStream::new();
    for tok in toks {
        st.extend(tok);
    }
    Ok(st)
}
