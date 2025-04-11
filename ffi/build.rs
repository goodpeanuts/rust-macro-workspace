use rt::{get_func_meta, FfiDef};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

macro_rules! collect_metas {
    ($($meta:expr),*) => {
        vec![
            $(format!("{:#?}", $meta)),*
        ]
    };
}

fn main() {
    
    println!("cargo:rerun-if-changed=build.rs");

    let metas = collect_metas!(
        get_func_meta(playground::test_fn as usize),
        get_func_meta(playground::mod1::mod1_fn as usize),
        playground::AModel::meta()
    );

    let out_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let ffi_file_path = Path::new(&out_dir).join("src/spec");

    if !ffi_file_path.exists() {
        fs::File::create(&ffi_file_path).expect("Failed to create spec file");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open(&ffi_file_path)
        .expect("Failed to open spec file");

    for meta in metas {
        writeln!(file, "{}", meta).expect("Failed to write to spec file");
    }
}
