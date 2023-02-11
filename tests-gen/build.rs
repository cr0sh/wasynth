use std::path::PathBuf;

fn main() {
    let cases_dir = {
        let mut manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir.pop();
        manifest_dir.push("tests");
        manifest_dir.push("cases");
        manifest_dir
    };

    let cases_dir = cases_dir.to_str().expect("cannot convert path to &str");
    println!("cargo:rerun-if-changed={cases_dir}");
}
