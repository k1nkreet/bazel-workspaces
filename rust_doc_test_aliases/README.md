`rust_doc_test` doesn't compile when crate we are generating documentation for is using renaming dependencies in it's Cargo.toml

One can run

`cargo test --doc`

to demonstrate how it works for cargo, and run

`bazel test //...`

to reproduce the error in Bazel
