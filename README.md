# Metal

Prototype compiled language built in Rust with LLVM.

## Contributing

Currently, Metal is currently not open source software. Metal will eventually open up and
become fully open source, but for the time being, it's not open for contributions.

## Development

Use nightly Rust.

If you want to add a dependency, add it to the workspace Cargo.toml and then
in the crate that you want to use this dependency in specify that dependency as `dep_name = { workspace = true }`.
This makes updating deps much easier.
