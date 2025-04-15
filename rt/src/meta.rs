#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Ty {
    Enum,
    Model,
    Class,
    Method,
    Func,
    Callback,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Definition {
    pub name: &'static str,
    pub namespace: &'static str,
    pub ty: Ty,
}

pub type MetaFn = fn() -> &'static Meta;

#[derive(Debug, PartialEq, Eq)]
pub struct Meta {
    // 记录需要依赖的内容
    pub dep: &'static [MetaFn],
    // 当前的导出信息
    pub def: &'static [&'static Definition],
    pub ty: Ty,
}

impl Meta {
    pub fn debug_without_dep(&self) -> String {
        let def_str = self
            .def
            .iter()
            .map(|d| {
                format!(
                    "        Definition {{\n            name: \"{}\",\n            namespace: \"{}\",\n            ty: {:?},\n        }}",
                    d.name, d.namespace, d.ty
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            "Meta {{\n    ty: {:?},\n    def: [\n{}\n    ],\n}}",
            self.ty, def_str
        )
    }
}

pub trait FfiDef {
    // const fn meta() -> &'static Meta
    const META: &'static Meta;

    fn meta() -> &'static Meta {
        Self::META
    }
}

use std::collections::HashSet;
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

        for dep_fn in meta.dep {
            let dep_meta = dep_fn();
            dfs(dep_meta, visited, result);
        }

        result.push(meta);
    }

    for meta in metas {
        dfs(meta, &mut visited, &mut result);
    }

    result.sort();
    result
}

impl Ord for Definition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ty
            .cmp(&other.ty)
            .then_with(|| self.name.cmp(other.name))
            .then_with(|| self.namespace.cmp(other.namespace))
    }
}

impl PartialOrd for Definition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Meta {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ty
            .cmp(&other.ty)
            .then_with(|| self.def.cmp(other.def))
            .then_with(|| self.dep.cmp(other.dep))
    }
}
impl PartialOrd for Meta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
