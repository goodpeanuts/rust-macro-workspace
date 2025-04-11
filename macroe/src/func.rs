use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

pub fn func_wrapper(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;
    let func_name_str = func_name.to_string();
    let ctor_fn_name = format_ident!("__ctor_{}", func_name_str);
    let force_link = format_ident!("__force_link_{}", func_name_str);

    let expanded = quote! {
            #input

            #[::ctor::ctor]
            fn #ctor_fn_name() {
                let func_ptr = #func_name as usize;
                ::rt::FUNC_META_MAP.insert(func_ptr, &::rt::Meta {
                    deps: &[],
                    def: &[&::rt::Definition {
                        name: #func_name_str,
                        ty: concat!("[fn] ", module_path!()),
                    }],
                });
            }

            #[used]
            #[cfg_attr(target_os = "linux", link_section = ".data.keep")]
            #[cfg_attr(target_os = "macos", link_section = "__DATA,.data.keep")]
            #[allow(non_upper_case_globals)]
            #[no_mangle]
            static #force_link: extern "C" fn() = {
                extern "C" fn wrapper() {
                    let _ = #ctor_fn_name as usize;
                }
                wrapper
            };

            
        };

    TokenStream::from(expanded)
}
