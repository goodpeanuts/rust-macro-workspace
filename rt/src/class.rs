// use crate::Meta;
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

// #[ctor::ctor]
pub fn init_class_meta_map() {
    // for meta in __CLASS_META {
    //     let crate_name = meta.namespace.split("::").next().unwrap_or(meta.namespace);
    //     CLASS_META_MAP
    //         .entry((crate_name, meta.name))
    //         .and_modify(|existing_metas| {
    //             existing_metas.push(meta.meta);
    //         })
    //         .or_insert_with(|| vec![meta.meta]);
    // }
}

pub fn submit_class_meta(namespace: &'static str, struct_name: &'static str, meta: &'static Meta) {
    let namespace = namespace.split("::").next().unwrap_or(namespace);
    CLASS_META_MAP
        .entry((namespace, struct_name))
        .or_insert_with(|| meta);
}

pub fn get_impl_meta(namespace: &'static str, name: &'static str) -> &'static crate::Meta {
    let namespace = namespace.split("::").next().unwrap_or(namespace);

    CLASS_META_MAP.get(&(namespace, name)).unwrap_or_else(|| {
        panic!(
            "ClassMeta not found for namespace: {}, name: {}",
            namespace, name
        )
    });
    todo!()
}
