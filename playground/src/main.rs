#![feature(used_with_arg)]
use playground::mod1::AClass;
use rt::{get_func_meta, FfiDef};

#[macroe::callback]
pub trait ACallback {
    fn compute(&self, a: i32, b: i32) -> i32;
}

#[macroe::r#enum]
pub enum AEnum {
    A,
    B,
}

#[macroe::model]
pub struct AModel {
    pub a: i32,
    pub b: i32,
}

#[macroe::func]
fn test_fn() {
    println!("test fn");
}

fn main() {
    let a = AClass::new(1, 2);
    let _ = a.mul();
    // println!("{:#?}", <dyn ACallback as FfiDef>::META);
    // println!("{:#?}", <AEnum as FfiDef>::META);
    // println!("{:#?}", <AModel as FfiDef>::META);
    // println!("{:#?}", get_func_meta(test_fn as usize));
    println!("{:#?}", AClass::meta());
}
