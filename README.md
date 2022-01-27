# autoclap

Auto-propagate Cargo.toml infos (name, version, author, repo) into app.

## Why?

`Cargo.toml` is the single source of truth containing application attributes such as:
* name
* version
* repository where it's hosted
* author(s)

This information is usually duplicated in the app's `main`, leading to potentials inconsistencies (users forget to update it).

## Usage

```rust
fn main() {
    let app = autoclap();
}
```