# Macro for distributed meta collection

## Description

This repository contains a workspace for developing and testing Rust macros. It includes a sample macro implementation and a playground project to test and validate the macro's behavior.
The workspace is structured to facilitate rapid development and testing of macros, with a focus on ease of use and flexibility.

- The `macroe` crate contains the macro implementation, while the `playground` crate serves as a definition ground for the macro. The `playground` crate includes mock dependencies to simulate real-world usage of the macro.
- The `playground` crate is designed to be a simple and effective way to test the macro's behavior without needing to set up a complex project structure. It includes a basic example of how to use the macro, as well as a test suite to validate its functionality.
- The `rt` crate is a runtime crate that provides the necessary runtime support for the macro when the program runs.
- The `ffi` crate is a library that provides an entry point for generating and managing FFI-related metadata. It collects all metadata (Meta) and writes it to the `src/spec` file. The `ffi` crate is designed to be a simple and effective way to manage FFI-related metadata without needing to set up a complex project structure.

## Features

- ✅ **Distributed Definitions**: Supports macros distributed across multiple files and crates.
- ✅ **Selective Export**: Allows selective registration and export of types. Unregistered types will not be exported.
- ✅ **Dependency Handling**: Automatically includes spec information for unregistered but dependent types, with support for cross-crate dependencies.
- ✅ **Feature Toggle**: All macro code generation is controlled by a unified feature flag, enabling it only in crates that generate FFI export code.
- ❌ **Separated `impl` Blocks**: `impl` blocks not located in the same file as the struct definition may not be successfully exported.


## Structure

```
ffi                  # Provides an entry point for generating and managing FFI-related metadata.
├── build.rs         # Collects all metadata (Meta) and writes it to the src/spec file.
└── src
    ├── lib.rs
    ├── main.rs
    └── spec         # Stores the generated metadata.

macroe               # Provides a set of custom derive macros for generating metadata (Meta) and definitions (Definition).
└── src
    ├── callback.rs  # Defines the #[macroe::callback] macro for generating metadata for callback interfaces.
    ├── class.rs     # Defines the #[macroe::class] macro for generating class-related metadata for structs and impl blocks.
    ├── enum.rs      # Defines the #[macroe::enum] macro for generating metadata for enums.
    ├── func.rs      # Defines the #[macroe::func] macro for generating metadata for functions.
    ├── lib.rs       # Entry point for the macro library.
    └── model.rs     # Defines the #[macroe::model] macro for generating metadata for structs.

playground            # Defines enums, structs, classes, and functions that use macros from macroe to generate metadata.
└── src
    ├── lib.rs       # Entry point for testing macros.
    ├── mod1.rs
    ├── mod2.rs      
    └── mod3.rs

playground_dep        # Provides dependencies for the playground crate to test cross-crate macro capabilities.
├── Cargo.toml
└── src
    └── lib.rs

rt                    # Provides the core structures and methods for metadata (Meta) and definitions (Definition).
└── src
    ├── class.rs
    ├── func.rs
    ├── lib.rs
    └── meta.rs
```

## Design Overview

### General Approach

For types that need to be exported, a trait is generated. This trait contains the type's spec information and the spec of its dependencies. The trait also provides a method to retrieve the spec information. This design is applicable to `Model`, `Enum`, and `Callback`.

For `func`, since it does not have a concrete instance to implement a trait, we use `ctor` and macros to generate a `ctor` function for the annotated `fn`. The `#[ctor]` attribute automatically runs a function to register the `Meta` into a global `HashMap` (e.g., using `once_cell::sync::Lazy<HashMap<..>>` or `dashmap::DashMap`). Since each function pointer is unique and determined at compile time, the corresponding information can be retrieved from the map using the function pointer.

For `class`, which includes `impl` blocks that may be scattered across multiple files or crates, the map uses the struct's name and the crate path (retrieved via `module_path!`) as the namespace to form the map key. This allows all `impl` blocks of a struct to register their `Meta` into the global `HashMap` via `ctor` functions.

### Meta Type

The `Meta` type is designed to represent the spec description of the annotated code.

```rust
pub type MetaFn = fn() -> &'static Meta;

pub struct Meta {
    // Records the dependencies
    pub dep: &'static [MetaFn],
    // Current export information
    pub def: &'static [&'static Definition],
    pub ty: Ty,
}
```

### Unified Trait for Exported Types

A unified trait is implemented for types that need to export their spec information.

```rust
pub trait FfiDef {
    // Compile-time determined information
    const META: &'static Meta;

    // Retrieves the meta information of the object
    fn meta() -> &'static Meta {
        // Default implementation for model, enum, and callback
        Self::META
    }
}
```

### Challenges

Unused Symbols Removed by Linker:

If an impl block is located in a separate file from the struct definition, its content may be optimized out by the linker. Therefore, impl blocks not in the same file as the struct definition are not guaranteed to be successfully exported.



## License

This project is licensed under the MIT License.
