# autoclap

Auto-propagate Cargo.toml infos (name, version, author, repo) into app.

## Why?

`Cargo.toml` is the single source of truth containing application attributes such as:
* name
* version
* repository where it's hosted
* author(s)

This information is usually duplicated in the app's `main`, leading to potential inconsistencies (update in one place but not another).
Moreover, a crate unifying this would provide consistency across all apps using it.

## Usage

```rust
fn main() {
    let app = autoclap();
}
```