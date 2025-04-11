
// use crate::Meta;
use dashmap::DashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct ClassMeta {
    pub namespace: &'static str,
    pub name: &'static str,
    pub dep: Vec<(&'static str, &'static str)>,
    pub def: Vec<&'static crate::Definition>,
}

pub static CLASS_META_MAP: Lazy<DashMap<(&str, &str), ClassMeta>> = Lazy::new(DashMap::new);

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

pub fn submit_class_meta(mut class_meta: ClassMeta) {
    class_meta.namespace = class_meta
        .namespace
        .split("::")
        .next()
        .unwrap_or(class_meta.namespace);
    CLASS_META_MAP
        .entry((class_meta.namespace, class_meta.name))
        .and_modify(|existing_metas| {
            existing_metas.dep.extend(class_meta.dep.iter());
            existing_metas.def.extend(class_meta.def.iter());
        })
        .or_insert_with(|| class_meta);
}

pub fn get_class_meta(namespace: &'static str, name: &'static str) -> Option<&'static crate::Meta> {
    let class_meta = CLASS_META_MAP
        .get(&(namespace, name))
        .map(|v| v.value().clone());
    
    if let Some(class_meta) = class_meta {
        let mut deps = vec![];
        class_meta.dep.iter().for_each(|(namespace, name)| {
            get_class_meta(namespace, name).map(|meta| {
                deps.push(meta);
            });
        });

        let leaked_deps: &'static [&'static crate::Meta] = Box::leak(Box::new(deps));
        let leaked_defs: &'static [&'static crate::Definition] = Box::leak(Box::new(class_meta.def));

        Some(Box::leak(Box::new(crate::Meta {
            deps: leaked_deps,
            def: leaked_defs,
        })))
    } else {
        None
    }
}
