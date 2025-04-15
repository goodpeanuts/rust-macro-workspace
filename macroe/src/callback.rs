use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemTrait};

pub fn callback_wrapper(attr: TokenStream, item: TokenStream) -> TokenStream {
    let deps_attr = parse_macro_input!(attr as crate::MockDepAttr);
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    let trait_name_str = trait_name.to_string();

    // TODO 区分callback: <dyn #s::meta as FfiDef>::meta()
    let deps = deps_attr.deps.iter().map(|lit| {
        let s = format_ident!("{}", lit.value());
        quote! { <#s as ::rt::FfiDef>::meta }
    });

    let expanded = quote! {
        #input
        impl ::rt::FfiDef for dyn #trait_name {
            const META: &'static ::rt::Meta = &::rt::Meta {
                dep: &[#(#deps),*],
                def: &[&::rt::Definition {
                    name: #trait_name_str,
                    namespace: module_path!(),
                    ty: ::rt::Ty::Callback,
                }],
                ty: ::rt::Ty::Callback,
            };
        }
    };

    TokenStream::from(expanded)
}
