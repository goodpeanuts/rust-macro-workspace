#[macroe::class]
pub struct AClass {
    pub a: i32,
    pub b: i32,
}

#[macroe::class]
impl AClass {
    pub fn mul(&self) -> i32 {
        self.a * self.b
    }
}