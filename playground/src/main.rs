use macroe::ipc;

fn main() {
    let container = knowledge_ai::Foo::new();

    #[ipc]
    use knowledge_ai::ffi::export::{ipc::setup_ipc, ID};
}
