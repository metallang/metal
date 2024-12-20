# Metal

A prototype compiled programming language, striving to be an easy-to-use borrow checked, non-garbage-collected,
memory safe, robust programming language.

## Contributing

As an open source project, we welcome contributions! However, because the project is still in it's early days,
the development is rather turbulent. For that reason, we suggest you to first let us know of your intentions by
opening an issue, or [joining our Discord](https://discord.gg/fDCMSbgpsB) and saying "hello" there. Thanks!

## Development

### Rust

The Metal Compiler, analyzer, and surrounding tools are all built with Rust.
More specifically, you should use the most latest Rust nightly release as Metal only
targets supporting it specifically.

#### Adding Dependencies

When adding dependencies to Metal please make sure to only add the main dependency information (i.e. `tokio = { version = "1", features = ["full"] }`)
to the `Cargo.toml` in Metal's root directory. Then, when using the dependency in one of Metal's child crates, in lieu of adding it directly, only add it
as a workspace dependency (i.e. `tokio = { workspace = true }`).
