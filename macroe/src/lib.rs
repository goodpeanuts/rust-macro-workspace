use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error, ItemUse, Path, UseTree};

#[proc_macro_attribute]
pub fn ipc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemUse);
    let mut setup_ipc_path = None;
    let mut id_path = None;

    // 解析 `use` 语句的 AST
    extract_paths(
        &input.tree,
        "".to_string(),
        &mut setup_ipc_path,
        &mut id_path,
    );

    // 确保两个路径都解析到了
    if setup_ipc_path.is_none() || id_path.is_none() {
        return Error::new_spanned(
            &input,
            "Expected use statement to contain both `setup_ipc` and `ID`",
        )
        .to_compile_error()
        .into();
    }

    let setup_ipc = setup_ipc_path.unwrap();
    let id = id_path.unwrap();

    let output = quote! {
        {
            container
                .handle()
                .app_handle()
                .domain()
                .register_ipc_runtime(
                    #id,
                    container.handle().app_handle().to_owned(),
                    #setup_ipc
                );
        }
    };

    output.into()
}

/// 递归提取 `UseTree` 中 `setup_ipc` 和 `ID` 的完整路径
fn extract_paths(
    tree: &UseTree,
    base: String,
    ipc_path: &mut Option<Path>,
    id_path: &mut Option<Path>,
) {
    match tree {
        UseTree::Path(path) => {
            let new_base = format!("{}::{}", base, path.ident);
            extract_paths(&path.tree, new_base, ipc_path, id_path);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                extract_paths(item, base.clone(), ipc_path, id_path);
            }
        }
        UseTree::Name(name) => {
            if name.ident == "setup_ipc" {
                let path = syn::parse_str::<Path>(&format!("{base}::setup_ipc"))
                    .expect("Failed to parse path");
                *ipc_path = Some(path.clone());
            } else if name.ident == "ID" {
                let path =
                    syn::parse_str::<Path>(&format!("{base}::ID")).expect("Failed to parse path");
                *id_path = Some(path.clone());
            }
        }
        _ => {}
    }
}
