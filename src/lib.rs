#[macro_export]
macro_rules! autoclap {
    () => {
        Command::new(format!(
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
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
        use clap::{Arg, Command};
        let app = autoclap!();
        assert_eq!(
            app.get_name(),
            "autoclap 0.2.3 :: https://github.com/mihaigalos/autoclap/releases/tag/0.2.3"
        );
    }
    #[test]
    fn test_autoclap_description_works_when_typical() {
        use clap::{Arg, Command};
        let app = autoclap!();
        assert_eq!(
            app.get_about().unwrap().to_string(),
            "👏 Auto-propagate Cargo.toml infos (name, version, author, repo) into app."
        );
    }
}
