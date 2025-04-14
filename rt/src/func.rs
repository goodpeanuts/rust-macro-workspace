use crate::meta::Meta;
use dashmap::DashMap;
use once_cell::sync::Lazy;

pub static FUNC_META_MAP: Lazy<DashMap<usize, &'static Meta>> = Lazy::new(DashMap::new);

pub fn get_func_meta_map() -> String {
    let mut info = String::new();
    info.push_str("\n=========== func ==============\n");
    for entry in FUNC_META_MAP.iter() {
        info.push_str(&format!(
            "key: {:#x}, value: {:#?}",
            entry.key(),
            entry.value()
        ));
    }
    info.push_str("\n=========== func ==============\n");
    info
}

pub fn get_func_meta(ptr: usize) -> Vec<&'static Meta> {
    let meta = FUNC_META_MAP
        .get(&ptr)
        .map(|v| *v)
        .unwrap_or_else(|| panic!("get func meta error, ptr: {:#x}", ptr));
    vec![meta]
}
