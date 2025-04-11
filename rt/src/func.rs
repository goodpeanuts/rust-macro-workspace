use crate::meta::Meta;
use dashmap::DashMap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct FuncMeta {
    pub namespace: &'static str,
    pub func: fn(),
    pub meta: &'static Meta,
}

pub static FUNC_META_MAP: Lazy<DashMap<usize, &'static Meta>> = Lazy::new(DashMap::new);

#[allow(unused)]
fn print_func_meta() {
    for entry in FUNC_META_MAP.iter() {
        println!("key: {:#x}, value: {:?}", entry.key(), entry.value());
    }
}

pub fn get_func_meta(ptr: usize) -> Option<&'static Meta> {
    FUNC_META_MAP.get(&ptr).map(|v| *v)
}
