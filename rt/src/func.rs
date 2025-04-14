use crate::meta::Meta;
use dashmap::DashMap;
use once_cell::sync::Lazy;

pub static FUNC_META_MAP: Lazy<DashMap<usize, &'static Meta>> = Lazy::new(DashMap::new);

pub fn get_func_meta_map() -> String {
    let mut info = String::new();
    for entry in FUNC_META_MAP.iter() {
        info.push_str(&format!(
            "key: {:#x}, value: {:#?}",
            entry.key(),
            entry.value()
        ));
    }
    info
}

pub fn get_func_meta(ptr: usize) -> &'static Meta {
    FUNC_META_MAP
        .get(&ptr)
        .map(|v| *v)
        .unwrap_or_else(|| panic!("get func meta error, ptr: {:#x}", ptr))
}
