use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::Meta;

#[derive(Debug, Clone)]
pub struct ClassMeta {
    pub dep: Vec<crate::MetaFn>,
    pub def: Vec<&'static crate::Definition>,
}

pub static CLASS_META_MAP: Lazy<DashMap<(&str, &str), ClassMeta>> = Lazy::new(DashMap::new);

pub fn submit_class_meta(
    namespace: &'static str,
    struct_name: &'static str,
    class_meta: ClassMeta,
) {
    let namespace = namespace.split("::").next().unwrap_or(namespace);
    CLASS_META_MAP
        .entry((namespace, struct_name))
        .and_modify(|m| {
            m.dep.extend(class_meta.dep.clone());
            m.def.extend(class_meta.def.clone());
        })
        .or_insert_with(|| class_meta.clone());
}

pub fn get_class_meta(namespace: &'static str, name: &'static str) -> &'static Meta {
    let namespace = namespace.split("::").next().unwrap_or(namespace);

    let meta = CLASS_META_MAP.get(&(namespace, name)).unwrap_or_else(|| {
        panic!(
            "ClassMeta not found for entry [namespace: {}, name: {}]\n{}",
            namespace,
            name,
            get_class_meta_map()
        )
    });

    let meta = Meta {
        dep: Box::leak(Box::new(meta.dep.clone())),
        def: Box::leak(Box::new(meta.def.clone())),
        ty: crate::Ty::Class,
    };
    Box::leak(Box::new(meta))
}

pub fn get_class_meta_map() -> String {
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
