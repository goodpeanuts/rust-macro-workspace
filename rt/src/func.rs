use crate::meta::Meta;
use dashmap::DashMap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct FuncMeta {
    pub namespace: &'static str,
    pub func: fn(),
    pub meta: &'static Meta,
}
#[linkme::distributed_slice]
pub static __FUNC_META: [&'static FuncMeta] = [..];

pub static FUNC_META_MAP: Lazy<DashMap<usize, &'static Meta>> = Lazy::new(DashMap::new);

#[ctor::ctor]
fn init_func_meta_map() {
    for meta in __FUNC_META {
        FUNC_META_MAP.insert(meta.func as usize, meta.meta);
    }
}

pub fn get_func_meta(ptr: usize) -> Option<&'static Meta> {
    FUNC_META_MAP.get(&ptr).map(|v| *v)
}
