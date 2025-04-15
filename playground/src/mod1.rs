#[macroe::r#enum]
pub enum AEnum {
    A,
    B,
}

use crate::mod2::{BEnum, BModel};
#[macroe::model("BModel", "BEnum")]
pub struct AModel {
    pub a: i32,
    pub b: i32,
    pub dep_enum: BEnum,
    pub dep_model: BModel,
}

#[macroe::class("crate::mod2::BClass")]
pub struct AClass {
    pub a: i32,
    pub b: i32,
    // pub c: ,
    pub dep_class: crate::mod2::BClass,
}

#[macroe::class]
impl AClass {
    pub fn mul(&self) -> i32 {
        self.a * self.b
    }
}

#[macroe::func]
pub fn mod1_fn() {
    println!("mod1 fn");
}

#[macroe::callback]
pub trait ACallback {
    fn compute(&self, a: i32, b: i32) -> i32;
}
