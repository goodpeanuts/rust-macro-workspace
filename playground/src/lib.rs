pub use mod1::AClass;

pub mod mod1;
pub mod mod2;
pub mod mod3;

#[macroe::class]
impl AClass {
    pub fn new(a: i32, b: i32, dep_class: crate::mod2::BClass) -> Self {
        AClass { a, b, dep_class }
    }

    pub fn add(&self) -> i32 {
        self.a + self.b
    }

    pub fn sub(&self) -> i32 {
        self.a - self.b
    }
}

#[macroe::func]
pub fn test_fn() {
    println!("test fn");
}
