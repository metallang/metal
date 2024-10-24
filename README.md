# Metal

A programming language. We'll add a catchy slogan soon, pinky swear

## Development

Use nightly Rust.

If you want to add a dependency, add it to the workspace `Cargo.toml` and then
in the crate that you want to use this dependency in specify that dependency as
`dep_name = { workspace = true }`. This makes updating deps much easier.
