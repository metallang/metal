# Metal

A beautiful, robust, flexible, efficient general-purpose programming language with zero garbage collection built with LLVM and Rust.

## Contributing

As an open source project, we welcome contributions! However, because the project is still in it's early days,
the development is rather turbulent. For that reason, we suggest you to first let us know of your intentions by
opening an issue, or [joining our Discord](https://discord.gg/fDCMSbgpsB) and saying "hello" there. Thanks!

## Development

Use nightly Rust.

If you want to add a dependency, add it to the workspace Cargo.toml and then
in the crate that you want to use this dependency in specify that dependency as `dep_name = { workspace = true }`.
This makes updating deps much easier.
