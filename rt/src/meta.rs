#[derive(Debug)]
pub struct Definition {
    pub name: &'static str,
    pub ty: &'static str,
}

#[derive(Debug)]
pub struct Meta {
    // 记录需要依赖的内容
    pub deps: &'static [&'static Meta],
    // 当前的导出信息
    pub def: Definition,
}

pub trait FfiDef {
    // const fn meta() -> &'static Meta
    const META: &'static Meta;
}
