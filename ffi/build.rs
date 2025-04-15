use rt::{get_func_meta, FfiDef, Meta};
use std::collections::HashSet;
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
        playground::AEnum::meta(),
        // model
        playground::AModel::meta(),
        // callback
        <dyn playground::ACallback>::meta(),
        // fn
        get_func_meta(playground::test_fn as usize),
        get_func_meta(playground::mod1::mod1_fn as usize),
        // class
        playground::AClass::meta()
    );

    let defs = collect_all_meta(metas);

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

/// 深度优先地收集所有依赖的 Meta，结果去重，按拓扑顺序输出
pub fn collect_all_meta(metas: Vec<&'static Meta>) -> Vec<&'static Meta> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    fn dfs(
        meta: &'static Meta,
        visited: &mut HashSet<*const Meta>,
        result: &mut Vec<&'static Meta>,
    ) {
        let ptr = meta as *const Meta;
        if visited.contains(&ptr) {
            return;
        }
        visited.insert(ptr);

        // 先递归处理依赖
        for dep_fn in meta.dep {
            let dep_meta = dep_fn();
            dfs(dep_meta, visited, result);
        }

        // 然后把当前 meta 加入结果
        result.push(meta);
    }

    for meta in metas {
        dfs(meta, &mut visited, &mut result);
    }

    result.sort();
    result
}
