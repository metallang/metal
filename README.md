# Metal

Prototype compiled language built in Rust with LLVM. We'll add a catchy slogan later.

## Contributing

Currently, Metal is more or less under source availability. Metal will eventually
become fully open source, but for now, since we're the only ones using this language it's unlicensed.

## Development

Use nightly Rust.

If you want to add a dependency, add it to the workspace Cargo.toml and then in the crate that you want to use this dependency in specify that dependency as dep_name = { workspace = true }. This makes updating deps much easier.
