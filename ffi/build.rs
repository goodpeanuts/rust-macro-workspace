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
        // enum
        // playground::AEnum::meta(),
        // model
        // playground::AModel::meta(),
        // callback
        // <dyn playground::ACallback>::meta(),
        // fn
        get_func_meta(playground::test_fn as usize),
        // get_func_meta(playground::mod1::mod1_fn as usize),
        // class
        playground::AClass::meta()
    )
    .into_iter()
    .flat_map(|meta| meta.iter().cloned().collect::<Vec<_>>())
    .collect();

    let mut defs = Vec::new();

    for meta in metas {
        if meta.deps.is_empty() {
            defs.extend_from_slice(meta.def);
        } else {
            for dep in meta.deps {
                let dep_meta = dep();
                dep_meta.iter().for_each(|dep_meta| {
                    defs.extend_from_slice(dep_meta.def);
                });
            }
            defs.extend_from_slice(meta.def);
        }
    }
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
        writeln!(file, "{:#?}", def).expect("Failed to write to spec file");
    }


    // writeln!(file, "{}", rt::get_func_meta_map()).expect("Failed to write to spec file");

    writeln!(file, "{}", rt::get_impl_meta_map()).expect("Failed to write to spec file");
}
