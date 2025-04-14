use proc_macro::TokenStream;

mod callback;
mod class;
mod r#enum;
mod func;
mod model;

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    model::model_wrapper(item)
}

#[proc_macro_attribute]
pub fn r#enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    r#enum::enum_wrapper(item)
}

#[proc_macro_attribute]
pub fn callback(_attr: TokenStream, item: TokenStream) -> TokenStream {
    callback::callback_wrapper(item)
}

#[proc_macro_attribute]
pub fn func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    func::func_wrapper(item)
}

#[proc_macro_attribute]
pub fn class(attr: TokenStream, item: TokenStream) -> TokenStream {
    class::class_wrapper(attr, item)
}
