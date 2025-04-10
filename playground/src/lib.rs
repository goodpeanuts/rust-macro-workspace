#![feature(used_with_arg)]
use mod1::AClass;

pub mod mod1;

#[macroe::class]
impl AClass {
    pub fn new(a: i32, b: i32) -> Self {
        AClass { a, b }
    }

    pub fn add(&self) -> i32 {
        self.a + self.b
    }

    pub fn sub(&self) -> i32 {
        self.a - self.b
    }
}