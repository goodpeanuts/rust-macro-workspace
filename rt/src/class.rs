use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::Meta;

#[derive(Debug, Clone)]
pub struct ClassMeta {
    pub namespace: &'static str,
    pub name: &'static str,
    pub dep: &'static [crate::MetaFn],
    pub def: &'static [&'static crate::Definition],
}

pub static CLASS_META_MAP: Lazy<DashMap<(&str, &str), &'static Meta>> = Lazy::new(DashMap::new);

pub fn get_impl_meta_map() -> String {
    let mut info = String::new();
    info.push_str("\n=========== impl ==============\n");
    for entry in CLASS_META_MAP.iter() {
        info.push_str(&format!(
            "key: {:#?}, value: {:#?}",
            entry.key(),
            entry.value()
        ));
    }
    info.push_str("\n=========== impl ==============\n");
    info
}

pub fn submit_class_meta(namespace: &'static str, struct_name: &'static str, meta: &'static Meta) {
    let namespace = namespace.split("::").next().unwrap_or(namespace);
    CLASS_META_MAP
        .entry((namespace, struct_name))
        .or_insert_with(|| meta);
}

pub fn get_impl_meta(namespace: &'static str, name: &'static str) -> &'static Meta {
    let namespace = namespace.split("::").next().unwrap_or(namespace);

    let meta = CLASS_META_MAP.get(&(namespace, name)).unwrap_or_else(|| {
        panic!(
            "ClassMeta not found for entry [namespace: {}, name: {}]\n{}",
            namespace, name, get_impl_meta_map()
        )
    });
    *meta
}
