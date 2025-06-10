use {{macroe}}::ipc;

fn main() {
    let container = deps::Foo::new();

    #[ipc]
    use deps::ffi::export::{ipc::setup_ipc, ID};
}
