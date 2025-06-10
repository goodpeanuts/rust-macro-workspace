# Rust Macro Workspace

This repository serves as a scaffold for quickly writing and testing Rust macros. It provides a structured workspace with the following components:

- **deps**: A mock dependency used for testing and demonstrating macro functionality.
- **{{macroe}}**: A library containing procedural macros for Rust.
- **{{playground}}**: A sample application to validate and experiment with the macros.

## Features

- Pre-configured workspace for Rust macro development.
- Example procedural macro implementation.
- Mock dependencies for testing macro behavior.
- Easy-to-use {{playground}} for rapid iteration and debugging.

## Usage

### Use cargo-generate

To use this workspace, you can clone the repository and build the components. The `{{playground}}` project is set up to demonstrate how to use the macros defined in the `{{macroe}}` crate.

Clone the repository using `cargo-generate`:

```bash
cargo generate --git https://github.com/goodpeanuts/rust-macro-workspace.git
```

### Use default template

if you prefer to use the default template without `cargo-generate`, you can follow these steps:

1. Clone the repository:
   ```bash
   git clone -b default https://github.com/goodpeanuts/rust-macro-workspace.git
   ```

2. Build the workspace:
   ```bash
   cargo build
   ```

3. Test the macros in the `{{playground}}` project:
   ```bash
   cargo test
   ```

4. Modify the `{{macroe}}` crate to add or update macros, and use the `{{playground}}` project and command `cargo expand` in `{{playground}}` to validate them.

```bash
cargo expand
```

## License

This project is licensed under the MIT License.
