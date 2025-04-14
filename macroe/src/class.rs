use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Ident, Item, ItemStruct, LitInt, Token,
};

struct ClassAttr {
    key: Ident,
    _eq_token: Token![=],
    value: LitInt,
}

impl Parse for ClassAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ClassAttr {
            key: input.parse()?,
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub fn class_wrapper(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    let class_attr = parse_macro_input!(attr as ClassAttr);

    match input {
        Item::Struct(item_struct) => {
            if class_attr.key != "impls" {
                return TokenStream::from(quote! {
                    compile_error!("Expected `impls` attribute");
                });
            }
            let impls_count = class_attr.value.base10_parse::<usize>().unwrap_or(0);
            handle_struct(item_struct, impls_count)
        }
        Item::Impl(item_impl) => {
            if class_attr.key != "id" {
                return TokenStream::from(quote! {
                    compile_error!("Expected `impl` attribute");
                });
            }
            let impl_idx = class_attr.value.base10_parse::<usize>().unwrap_or(0);
            handle_impl(item_impl, impl_idx)
        }
        _ => {
            // 如果输入不是 `struct` 或 `impl`，返回一个编译错误
            TokenStream::from(quote! {
                compile_error!("Expected a struct or impl block");
            })
        }
    }
}

fn handle_struct(item_struct: ItemStruct, impls_count: usize) -> TokenStream {
    let struct_name = &item_struct.ident;
    let struct_name_str = struct_name.to_string();
    let get_impl_code_fn = format_ident!("__{}_get_impl", struct_name_str);

    let impl_code = (0..=impls_count).map(|i| {
        let impl_name = format!("__{}_impl{}", struct_name_str, i);
        quote! {
            ::rt::get_impl_meta(module_path!(), #impl_name)
        }
    });

    let expanded = quote! {
        #item_struct
        
        fn #get_impl_code_fn() -> Vec<&'static ::rt::Meta> {
            vec![
                #(
                    #impl_code,
                )*
            ]
        }
        impl ::rt::FfiDef for #struct_name {
            const META: &'static ::rt::Meta = &::rt::Meta {
                deps: &[
                    #get_impl_code_fn,
                ],
                def: &[
                    &::rt::Definition {
                        name: #struct_name_str,
                        ty: concat!("[class] ", module_path!()),
                    },
                ]
                
            };
            fn meta() -> Vec<&'static ::rt::Meta> {
                vec![Self::META]
            }
        }
    };

    TokenStream::from(expanded)
}
fn handle_impl(item_impl: syn::ItemImpl, impl_idx: usize) -> TokenStream {
    let struct_name = item_impl.get_name().expect("get impl class name error");
    let mut methods = Vec::new();
    let uuid = uuid::Uuid::new_v4().to_u128_le();
    let ctor_name = format_ident!("__IMPL_META_CTOR_{}_{}", struct_name, uuid);
    let impl_name = format!("__{}_impl{}", struct_name, impl_idx);

    for item in &item_impl.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();

            methods.push(quote! {
                &::rt::Definition {
                    name: #method_name_str,
                    ty: concat!("[class_impl] ", module_path!()),
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

        #[::ctor::ctor]
        pub fn #ctor_name() {
            ::rt::submit_class_meta(
                module_path!(),
                #impl_name,
                &::rt::Meta {
                    deps: &[],
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
