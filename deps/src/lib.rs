pub struct Foo;

impl Foo {
    pub fn new() -> Self {
        Self
    }
    pub fn handle(&self) -> Self {
        Self
    }

    pub fn app_handle(&self) -> Self {
        Self
    }
    pub fn domain(&self) -> Self {
        Self
    }
    pub fn register_ipc_runtime(&self, _: u64, _: Self, _: fn()) -> Self {
        Self
    }
}

impl Default for Foo {
    fn default() -> Self {
        Self::new()
    }
}

impl ToOwned for Foo {
    type Owned = Self;
    fn to_owned(&self) -> Self::Owned {
        Self
    }
}

pub mod ffi {
    pub mod export {
        pub mod ipc {
            pub fn setup_ipc() {}
        }
        pub const ID: u64 = 1;
    }
}
