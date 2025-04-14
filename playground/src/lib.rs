#![feature(used_with_arg)]
#![feature(const_mut_refs)]
// use mod1::AClass;

pub mod mod1;

// #[macroe::class]
// impl AClass {
//     pub fn new(a: i32, b: i32) -> Self {
//         AClass { a, b }
//     }

//     pub fn add(&self) -> i32 {
//         self.a + self.b
//     }

//     pub fn sub(&self) -> i32 {
//         self.a - self.b
//     }
// }

#[macroe::callback]
pub trait ACallback {
    fn compute(&self, a: i32, b: i32) -> i32;
}

#[macroe::r#enum]
pub enum AEnum {
    A,
    B,
}

#[macroe::model("AEnum")]
pub struct AModel {
    pub a: i32,
    pub b: i32,
}


#[macroe::func]
pub fn test_fn() {
    println!("test fn");
}
