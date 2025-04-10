use rt::get_func_meta;
pub use rt::meta::FfiDef;

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
    println!("{:?}", <dyn ACallback>::META);
    println!("{:?}", <AEnum>::META);
    println!("{:?}", <AModel>::META);
    println!("{:?}", get_func_meta(test_fn as usize));
}

