# autoclap
🚧 WIP 🚧

This lib builds on top of [`clap`](https://crates.io/crates/clap) and auto-propagates `Cargo.toml` infos (name, version, author, repo) into the application using it.

## Why?

`Cargo.toml` is the single source of truth containing application attributes such as:
* name
* version

Once `clap :: author<S: Into<String>>(self, author: S)` and `clap :: about<S: Into<String>>(self, author: S)` are implemented, those can be implicitly deduced as well.

This information is usually duplicated in the app's `main`, leading to potential inconsistencies (update in one place but not another).

Moreover, a crate unifying this would provide consistency across all apps using it.

## Usage

```rust
fn main() {
    let app = autoclap(); // return a clap::App
    app.arg(
        Arg::with_name("myarg")
            .long("myarg")
            .short('m')
            .help("My arg description."),
    )
    .get_matches_safe()
    .unwrap_or_else(|e| e.exit());
}
```

## Releases
The packge repository URL is concatenated with the version of a package to form a URL to a specific release.

This URL goes live when a release with the same version as the crate is published.

`autoclap` offers out-of-the box support for `GitHub` and `GitLab`.
