use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, Item, ItemImpl, ItemStruct};

// struct ClassAttr {
//     key: Ident,
//     _eq_token: Token![=],
//     value: LitInt,
// }

// impl Parse for ClassAttr {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         Ok(ClassAttr {
//             key: input.parse()?,
//             _eq_token: input.parse()?,
//             value: input.parse()?,
//         })
//     }
// }

pub fn class_wrapper(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    match input {
        Item::Struct(item_struct) => handle_struct(attr, item_struct),
        Item::Impl(item_impl) => handle_impl(attr, item_impl),
        _ => TokenStream::from(quote! {
            compile_error!("Expected a struct or impl block");
        }),
    }
}

fn handle_struct(attr: TokenStream, item_struct: ItemStruct) -> TokenStream {
    let struct_name = &item_struct.ident;
    let struct_name_str = struct_name.to_string();
    let struct_name_lower = struct_name_str.to_lowercase();

    let uuid = uuid::Uuid::new_v4().to_u128_le();
    let ctor_fn_name = format_ident!("__ctor_class_{}_{}", struct_name_lower, uuid);

    let deps_attr = parse_macro_input!(attr as crate::MockDepAttr);

    // TODO 区分callback: <dyn #s::meta as FfiDef>::meta()
    let deps = deps_attr
        .deps
        .iter()
        .map(|lit| {
            let s: syn::Path = syn::parse_str(&lit.value()).expect("Invalid path");
            quote! { <#s as ::rt::FfiDef>::meta }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #item_struct

        impl ::rt::FfiDef for #struct_name {
            const META: &'static ::rt::Meta = &::rt::Meta {
                dep: &[#(#deps),*],
                def: &[&::rt::Definition {
                    name: #struct_name_str,
                    namespace: module_path!(),
                    ty: ::rt::Ty::Class,
                }],
                ty: ::rt::Ty::Class,
            };
            fn meta() -> &'static ::rt::Meta {
                ::rt::get_class_meta(module_path!(), #struct_name_str)
            }
        }

        #[::ctor::ctor]
        pub fn #ctor_fn_name() {
            ::rt::submit_class_meta(
                module_path!(),
                #struct_name_str,
                ::rt::ClassMeta {
                    dep: vec![#(#deps),*],
                    def: vec![&::rt::Definition {
                        name: #struct_name_str,
                        namespace: module_path!(),
                        ty: ::rt::Ty::Class,
                    }],
                }
            );
        }

    };

    TokenStream::from(expanded)
}
fn handle_impl(attr: TokenStream, item_impl: ItemImpl) -> TokenStream {
    let struct_name_str = item_impl.get_name().expect("get impl class name error");
    let struct_name_lower = struct_name_str.to_lowercase();
    let mut methods = Vec::new();
    let uuid = uuid::Uuid::new_v4().to_u128_le();
    let ctor_fn_name = format_ident!("__ctor_impl_{}_{}", struct_name_lower, uuid);

    let deps_attr = parse_macro_input!(attr as crate::MockDepAttr);

    // TODO 区分callback: <dyn #s::meta as FfiDef>::meta()
    let deps = deps_attr.deps.iter().map(|lit| {
        let s: syn::Path = syn::parse_str(&lit.value()).expect("Invalid path");
        quote! { <#s as ::rt::FfiDef>::meta }
    });

    for item in &item_impl.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();

            methods.push(quote! {
                &::rt::Definition {
                    name: #method_name_str,
                    namespace: module_path!(),
                    ty: ::rt::Ty::Method,
                }
            });
        }
    }

    // 将 methods 转换为 `def` 数组
    let def = quote! {
        vec![#(#methods),*]
    };
    let expanded = quote! {
        #item_impl

        #[::ctor::ctor]
        pub fn #ctor_fn_name() {
            ::rt::submit_class_meta(
                module_path!(),
                #struct_name_str,
                ::rt::ClassMeta {
                    dep: vec![#(#deps),*],
                    def: #def,
                }
            );
        }
    };

    TokenStream::from(expanded)
}

pub trait GeneralSynItem {
    fn get_name(&self) -> syn::Result<String>;
    #[allow(unused)]
    fn get_span(&self) -> proc_macro2::Span;
}

impl GeneralSynItem for syn::ItemImpl {
    fn get_name(&self) -> syn::Result<String> {
        if let syn::Type::Path(path) = *(self.self_ty.clone()) {
            if path.path.segments.is_empty() {
                Err(syn::Error::new(self.span(), "unsupported impl\n"))?
            }
            Ok(path.path.segments.last().unwrap().ident.to_string())
        } else {
            Err(syn::Error::new(self.span(), "unsupported impl\n"))
        }
    }
    fn get_span(&self) -> proc_macro2::Span {
        self.span()
    }
}
