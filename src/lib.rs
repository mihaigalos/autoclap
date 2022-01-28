use clap::{App, Arg};
use std::env;

pub fn autoclap() -> clap::App<'static> {
    let repo = env::var("CARGO_PKG_REPOSITORY").unwrap();
    let mut release_tag = "";

    if repo.contains("github") {
        release_tag = "/releases/tag/";
    } else if repo.contains("gitlab") {
        release_tag = "/-/releases/";
    }

    if repo.contains("github") {
        let app_name = format!(
            "{}{}{}{}{}",
            env::var("CARGO_PKG_NAME").unwrap(),
            " ",
            env::var("CARGO_PKG_VERSION").unwrap(),
            " :: ",
            format!(
                "{}{}{}",
                env::var("CARGO_PKG_REPOSITORY").unwrap(),
                release_tag,
                env::var("CARGO_PKG_VERSION").unwrap(),
            )
        );

        return App::new(app_name).arg(
            Arg::new("debug")
                .long("debug")
                .short('D')
                .help("Print raw data used internally."),
        );
    } else {
        panic!("Cannot determine repository from repository URL.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_autoclap_name_works_when_typical() {
        let app = autoclap();
        assert_eq!(
            app.get_name(),
            "autoclap 0.0.2 :: https://github.com/mihaigalos/autoclap/releases/tag/0.0.2"
        );
    }
    #[test]
    fn test_autoclap_author_works_when_typical() {
        let app = autoclap();
        assert_eq!(
            app.get_about().unwrap(),
            "👏 Auto-propagate Cargo.toml infos (name, version, author, repo) into app."
        );
    }
}
