use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait};

pub fn callback_wrapper(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    let trait_name_str = trait_name.to_string();

    let expanded = quote! {
        #input
        impl FfiDef for dyn #trait_name {
            const META: &'static ::rt::Meta = &::rt::Meta {
                deps: &[],
                def: ::rt::Definition {
                    name: #trait_name_str,
                    ty: concat!("dyn ", #trait_name_str),
                },
            };
        }
    };

    TokenStream::from(expanded)
}
