#[macroe::r#enum]
pub enum BEnum {
    D1,
    D2,
}

#[macroe::model]
pub struct BModel {
    pub a: i32,
    pub b: i32,
}

#[macroe::class]
pub struct BClass {
    pub a: i32,
    pub b: i32,
}

#[macroe::class]
impl BClass {
    pub fn new(a: i32, b: i32) -> Self {
        BClass { a, b }
    }
}
