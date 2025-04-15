use proc_macro::TokenStream;
#[cfg(feature = "bindgen")]
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Token,
};

#[cfg(feature = "bindgen")]
mod callback;
#[cfg(feature = "bindgen")]
mod class;
#[cfg(feature = "bindgen")]
mod r#enum;
#[cfg(feature = "bindgen")]
mod func;
#[cfg(feature = "bindgen")]
mod model;

#[cfg(feature = "bindgen")]
pub(crate) struct MockDepAttr {
    pub deps: Vec<LitStr>,
}
#[cfg(feature = "bindgen")]
impl Parse for MockDepAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punctuated = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        Ok(MockDepAttr {
            deps: punctuated.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "bindgen")]
    return model::model_wrapper(_attr, item);

    #[cfg(not(feature = "bindgen"))]
    return item;
}

#[proc_macro_attribute]
pub fn r#enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "bindgen")]
    return r#enum::enum_wrapper(_attr, item);
    #[cfg(not(feature = "bindgen"))]
    return item;
}

#[proc_macro_attribute]
pub fn callback(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "bindgen")]
    return callback::callback_wrapper(_attr, item);
    #[cfg(not(feature = "bindgen"))]
    return item;
}

#[proc_macro_attribute]
pub fn func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "bindgen")]
    return func::func_wrapper(_attr, item);
    #[cfg(not(feature = "bindgen"))]
    return item;
}

#[proc_macro_attribute]
pub fn class(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "bindgen")]
    return class::class_wrapper(_attr, item);

    #[cfg(not(feature = "bindgen"))]
    return item;
}
