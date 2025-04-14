#[derive(Debug)]
pub struct Definition {
    pub name: &'static str,
    pub ty: &'static str,
}

pub type MetaFn = fn() -> Vec<&'static Meta>;

#[derive(Debug)]
pub struct Meta {
    // 记录需要依赖的内容
    pub deps: &'static [MetaFn],
    // 当前的导出信息
    pub def: &'static [&'static Definition],
}

pub trait FfiDef {
    // const fn meta() -> &'static Meta
    const META: &'static Meta;

    fn meta() -> Vec<&'static Meta> {
        vec![Self::META]
    }
}
