use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

pub fn func_wrapper(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;
    let func_name_str = func_name.to_string();
    let static_var_name = format_ident!("__FUNC_META_{}", func_name_str);

    let expanded = quote! {
        #input


        #[::linkme::distributed_slice(::rt::__FUNC_META)]
        static #static_var_name: &'static::rt::FuncMeta = &::rt::FuncMeta {
            namespace: module_path!(),
            func: #func_name,
            meta: &::rt::Meta {
                deps: &[],
                def: &[&::rt::Definition {
                    name: #func_name_str,
                    ty: concat!("func::", module_path!()),
                }],
            },
        };

    };

    TokenStream::from(expanded)
}
