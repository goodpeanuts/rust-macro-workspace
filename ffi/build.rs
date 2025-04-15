use rt::{get_func_meta, FfiDef, Meta};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

macro_rules! collect_metas {
    ($($meta:expr),* $(,)?) => {
        vec![
            $($meta),*
        ]
    };
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let metas: Vec<&'static Meta> = collect_metas!(
        playground::mod1::AEnum::meta(),
        playground::mod1::AModel::meta(),
        <dyn playground::mod1::ACallback>::meta(),
        get_func_meta(playground::test_fn as usize),
        get_func_meta(playground::mod1::mod1_fn as usize),
        playground::mod3::_ExportModel::meta(),
        playground::AClass::meta()
    );

    let defs = rt::collect_all_meta(metas);

    let out_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let ffi_file_path = Path::new(&out_dir).join("src/spec");

    if ffi_file_path.exists() {
        fs::remove_file(&ffi_file_path).expect("Failed to remove existing spec file");
    }
    fs::File::create(&ffi_file_path).expect("Failed to create spec file");

    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open(&ffi_file_path)
        .expect("Failed to open spec file");

    for def in defs {
        writeln!(file, "{:}", def.debug_without_dep()).expect("Failed to write to spec file");
    }

    // writeln!(file, "{}", rt::get_func_meta_map()).expect("Failed to write to spec file");

    // writeln!(file, "{}", rt::get_class_meta_map()).expect("Failed to write to spec file");
}
