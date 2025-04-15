use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Token,
};

mod callback;
mod class;
mod r#enum;
mod func;
mod model;

pub(crate) struct MockDepAttr {
    pub deps: Vec<LitStr>,
}

impl Parse for MockDepAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punctuated = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        Ok(MockDepAttr {
            deps: punctuated.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn model(attr: TokenStream, item: TokenStream) -> TokenStream {
    model::model_wrapper(attr, item)
}

#[proc_macro_attribute]
pub fn r#enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    r#enum::enum_wrapper(attr, item)
}

#[proc_macro_attribute]
pub fn callback(attr: TokenStream, item: TokenStream) -> TokenStream {
    callback::callback_wrapper(attr, item)
}

#[proc_macro_attribute]
pub fn func(attr: TokenStream, item: TokenStream) -> TokenStream {
    func::func_wrapper(attr, item)
}

#[proc_macro_attribute]
pub fn class(attr: TokenStream, item: TokenStream) -> TokenStream {
    class::class_wrapper(attr, item)
}
