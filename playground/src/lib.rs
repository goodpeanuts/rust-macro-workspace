pub use mod1::AClass;

pub mod mod1;
pub mod mod2;
pub mod mod3;

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


// 这个文件中若只有impl标记#[macroe::class]，其信息会被编译优化而丢弃
// 如果还有其他非impl的导出，比如#[macroe::func]，则不会被优化掉
#[macroe::func]
pub fn test_fn() {
    println!("test fn");
}
