use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

pub fn enum_wrapper(attr: TokenStream, item: TokenStream) -> TokenStream {
    let deps_attr = parse_macro_input!(attr as crate::MockDepAttr);
    let input = parse_macro_input!(item as ItemEnum);
    let trait_name = &input.ident;
    let trait_name_str = trait_name.to_string();

    // TODO 区分callback: <dyn #s::meta as FfiDef>::meta()
    let deps = deps_attr.deps.iter().map(|lit| {
        let s: syn::Path = syn::parse_str(&lit.value()).expect("Invalid path");
        quote! { <#s as ::rt::FfiDef>::meta }
    });

    let expanded = quote! {
        #input

        impl ::rt::FfiDef for #trait_name {
            const META: &'static ::rt::Meta = &::rt::Meta {
                dep: &[#(#deps),*],
                def: &[&::rt::Definition {
                    name: #trait_name_str,
                    namespace: module_path!(),
                    ty: ::rt::Ty::Enum,
                }],
                ty: ::rt::Ty::Enum,
            };
        }
    };

    TokenStream::from(expanded)
}
