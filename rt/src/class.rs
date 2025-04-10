use crate::Meta;
use dashmap::DashMap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct ClassMeta {
    pub namespace: &'static str,
    pub name: &'static str,
    pub meta: &'static Meta,
}

#[linkme::distributed_slice]
pub static __CLASS_META: [ClassMeta];


pub static CLASS_META_MAP: Lazy<DashMap<(&str, &str), Vec<&'static Meta>>> =
    Lazy::new(DashMap::new);

#[ctor::ctor]
fn init_class_meta_map() {
    println!("===== section  =====");
    println!("{__CLASS_META:#?}");
    println!("===== section  =====");
    for meta in __CLASS_META {
        let crate_name = meta.namespace.split("::").next().unwrap_or(meta.namespace);
        CLASS_META_MAP
            .entry((crate_name, meta.name))
            .and_modify(|existing_metas| {
                existing_metas.push(meta.meta);
            })
            .or_insert_with(|| vec![meta.meta]);
    }
}

pub fn get_class_meta(namespace: &'static str, name: &'static str) -> Option<Vec<&'static Meta>> {
    let namespace = namespace.split("::").next().unwrap_or(namespace);
    CLASS_META_MAP
        .get(&(namespace, name))
        .map(|v| v.clone().to_vec())
}
