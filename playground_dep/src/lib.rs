#[macroe::r#enum]
pub enum DepEnum {
    D1,
    D2,
}

#[macroe::model]
pub struct DepModel {
    pub a: i32,
    pub b: i32,
}

#[macroe::class]
pub struct DepClass {
    pub a: i32,
    pub b: i32,
}

#[macroe::class]
impl DepClass {
    pub fn hello(a: i32, b: i32) -> Self {
        DepClass { a, b }
    }
}

#[macroe::class]
impl DepClass {
    pub fn world(&self) -> i32 {
        self.a + self.b
    }
}
