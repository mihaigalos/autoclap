# autoclap

[![CD-auto](https://github.com/mihaigalos/autoclap/actions/workflows/cd-auto.yaml/badge.svg)](https://github.com/mihaigalos/autoclap/actions/workflows/cd-auto.yaml)

This lib builds on top of [`clap`](https://crates.io/crates/clap) and auto-propagates `Cargo.toml` infos (name, version, author, repo) into the application using it.

Running `app --help` now automatically includes this information in its output.

## Why?

`Cargo.toml` is the single source of truth containing application attributes such as:
* name
* version
* author(s)
* description

This information is usually duplicated in the app's `main`, leading to potential inconsistencies (update in one place but not another).

Moreover, a crate unifying this would provide consistency across all apps using it.

Finally, code for app initialization is minimal.

## Usage

```rust
use clap::{App, Arg};
use autoclap::autoclap;

fn main() {
    let app = autoclap!(); // return a clap::App

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

Now, running `app --help` now produces:

```
app 0.1.0 :: https://github.com/username/app/releases/tag/0.1.0
Crate Author <crateauthor@e.mail>
🧵 The crate awesome description.
```

## Releases
The packge repository URL is concatenated with the version of a package to form a URL to a specific release.

This URL goes live when a release with the same version as the crate is published.

`autoclap` offers out-of-the box support for `GitHub`.
