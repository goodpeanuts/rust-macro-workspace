use proc_macro::TokenStream;
use quote::{format_ident, quote};
use rand::prelude::*;
use syn::{parse_macro_input, spanned::Spanned, Item, ItemStruct};

pub fn class_wrapper(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    match input {
        Item::Struct(item_struct) => handle_struct(item_struct),
        Item::Impl(item_impl) => handle_impl(item_impl),
        _ => {
            // 如果输入不是 `struct` 或 `impl`，返回一个编译错误
            TokenStream::from(quote! {
                compile_error!("Expected a struct or impl block");
            })
        }
    }
}

fn handle_struct(item_struct: ItemStruct) -> TokenStream {
    let struct_name = &item_struct.ident;
    let struct_name_str = struct_name.to_string();
    let static_var_name = format_ident!("__CLASS_SELF_META_{}", struct_name_str);

    let expanded = quote! {
        #item_struct

        impl ::rt::FfiDef for #struct_name {
            const META: &'static ::rt::Meta = &::rt::Meta {
                deps: &[],
                def: &[&::rt::Definition {
                    name: #struct_name_str,
                    ty: concat!("[class] ", module_path!()),
                }],
            };
            fn meta() -> Vec<&'static ::rt::Meta> {
                for entry in ::rt::CLASS_META_MAP.iter() {
                    let ((namespace, name), metas) = entry.pair();
                    println!("Namespace: {}, Name: {}", namespace, name);
                    for meta in metas {
                        println!("  Meta: {:?}", meta);
                    }
                };
                ::rt::get_class_meta(module_path!(), #struct_name_str).unwrap()
            }
        }

        #[::linkme::distributed_slice(::rt::__CLASS_META)]
        static #static_var_name: ::rt::ClassMeta = ::rt::ClassMeta {
            namespace: module_path!(),
            name: #struct_name_str,
            meta: &::rt::Meta {
                deps: &[],
                def: &[&::rt::Definition {
                    name: #struct_name_str,
                    ty: concat!("[class] ", module_path!()),
                }],
            },

        };
    };

    TokenStream::from(expanded)
}
fn handle_impl(item_impl: syn::ItemImpl) -> TokenStream {
    let struct_name = item_impl.get_name().expect("get impl class name error");
    let mut methods = Vec::new();
    let uuid = uuid::Uuid::new_v4().as_u128();
    let ctor_name = format_ident!("__CLASS_META_CTOR_{}_{}", struct_name, uuid);
    let static_var_name = format_ident!("__CLASS_META_{}_{}", struct_name, uuid);

    for item in &item_impl.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();

            methods.push(quote! {
                &::rt::Definition {
                    name: #method_name_str,
                    ty: concat!("[class] ", module_path!()),
                }
            });
        }
    }

    // 将 methods 转换为 `def` 数组
    let def = quote! {
        &[#(#methods),*]
    };
    let expanded = quote! {
        #item_impl
        
        #[no_mangle]
        #[::linkme::distributed_slice(::rt::__CLASS_META)]
        static #static_var_name: ::rt::ClassMeta = ::rt::ClassMeta {
            namespace: module_path!(),
            name: #struct_name,
            meta: &::rt::Meta {
                deps: &[],
                def: #def,
            },
        };

        #[::ctor::ctor]
        #[allow(unused)]
        pub fn #ctor_name() {
            let t = (& #static_var_name) as *const _;
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
