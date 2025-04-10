use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

pub fn enum_wrapper(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let trait_name = &input.ident;
    let trait_name_str = trait_name.to_string();

    let expanded = quote! {
        #input
        impl ::rt::FfiDef for #trait_name {
            const META: &'static ::rt::Meta = &::rt::Meta {
                deps: &[],
                def: &[&::rt::Definition {
                    name: #trait_name_str,
                    ty: "enum",
                }],
            };
        }
    };

    TokenStream::from(expanded)
}
