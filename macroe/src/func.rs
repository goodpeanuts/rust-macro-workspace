use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

pub fn func_wrapper(attr: TokenStream, item: TokenStream) -> TokenStream {
    let deps_attr = parse_macro_input!(attr as crate::MockDepAttr);
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;
    let func_name_str = func_name.to_string();
    let uuid = uuid::Uuid::new_v4().to_u128_le();
    let ctor_fn_name = format_ident!("__ctor_{}_{}", func_name_str, uuid);
    // let force_link = format_ident!("__force_link_{}", func_name_str);

    // TODO 区分callback: <dyn #s::meta as FfiDef>::meta()
    let deps = deps_attr.deps.iter().map(|lit| {
        let s: syn::Path = syn::parse_str(&lit.value()).expect("Invalid path");
        quote! { <#s as ::rt::FfiDef>::meta }
    });

    let expanded = quote! {
        #input


        const _: () =  {
            #[::rt::deps::ctor::ctor]
            fn #ctor_fn_name() {
                let func_ptr = #func_name as usize;
                ::rt::FUNC_META_MAP.insert(func_ptr, &::rt::Meta {
                    dep: &[#(#deps),*],
                    def: &[&::rt::Definition {
                        name: #func_name_str,
                        namespace: module_path!(),
                        ty: ::rt::Ty::Func,
                    }],
                    ty: ::rt::Ty::Func,
                });
            }
        };


        // #[used]
        // #[cfg_attr(target_os = "linux", link_section = ".data.keep")]
        // #[cfg_attr(target_os = "macos", link_section = "__DATA,.data.keep")]
        // #[allow(non_upper_case_globals)]
        // #[no_mangle]
        // static #force_link: extern "C" fn() = {
        //     extern "C" fn wrapper() {
        //         let _ = #ctor_fn_name as usize;
        //     }
        //     wrapper
        // };


    };

    TokenStream::from(expanded)
}
