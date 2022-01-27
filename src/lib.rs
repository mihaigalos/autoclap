use clap::{App, Arg};

pub fn autoclap() -> clap::App<'static> {
    let repo = env!("CARGO_PKG_REPOSITORY");
    if repo.contains("github") {
        return App::new(concat!(
            env!("CARGO_CRATE_NAME"),
            " ",
            env!("CARGO_PKG_VERSION"),
            " :: ",
            concat!(
                env!("CARGO_PKG_REPOSITORY"),
                "/releases/tag/",
                env!("CARGO_PKG_VERSION"),
            )
        ))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Print raw data used in statistics."),
        );
    } else if repo.contains("gitlab") {
        return App::new(concat!(
            env!("CARGO_CRATE_NAME"),
            " ",
            env!("CARGO_PKG_VERSION"),
            " :: ",
            concat!(
                env!("CARGO_PKG_REPOSITORY"),
                "/-/releases/",
                env!("CARGO_PKG_VERSION"),
            )
        ))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Print raw data used in statistics."),
        );
    } else {
        panic!("Cannot determine repository.");
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
