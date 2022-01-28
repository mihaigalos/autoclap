#[macro_export]
macro_rules! autoclap {
    () => {
        App::new(format!(
            "{}{}{}{}{}",
            env!("CARGO_PKG_NAME"),
            " ",
            env!("CARGO_PKG_VERSION"),
            " :: ",
            format!(
                "{}{}{}",
                env!("CARGO_PKG_REPOSITORY"),
                "/releases/tag/",
                env!("CARGO_PKG_VERSION"),
            )
        ))
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('D')
                .help("Print raw data used internally."),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_autoclap_name_works_when_typical() {
        use clap::{App, Arg};
        let app = autoclap!();
        assert_eq!(
            app.get_name(),
            "autoclap 0.1.1 :: https://github.com/mihaigalos/autoclap/releases/tag/0.1.1"
        );
    }
    #[test]
    #[ignore] // Await clap :: author<S: Into<String>>(self, author: S)
    fn test_autoclap_author_works_when_typical() {
        use clap::{App, Arg};
        let app = autoclap!();
        assert_eq!(
            app.get_about().unwrap(),
            "👏 Auto-propagate Cargo.toml infos (name, version, author, repo) into app."
        );
    }
}
