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
    fn grep_version() -> String {
        use std::process::Command;
        let output = Command::new("tools/get_version.sh")
            .output()
            .expect("Failed to get version");

        String::from_utf8(output.stdout).expect("Found invalid UTF-8")
    }
    #[test]
    fn test_autoclap_name_works_when_typical() {
        use clap::{Arg, Command};
        let app = autoclap!();
        let version = grep_version();
        let expected = "autoclap ".to_string()
            + &version
            + " :: https://github.com/mihaigalos/autoclap/releases/tag/"
            + &version;

        let actual = app.get_name();

        assert_eq!(actual, expected);
    }
    #[test]
    fn test_autoclap_description_works_when_typical() {
        use clap::{Arg, Command};
        let expected = "👏 Auto-propagate Cargo.toml infos (name, version, author, repo) into app.";
        let app = autoclap!();

        let actual = app.get_about().unwrap().to_string();

        assert_eq!(actual, expected);
    }
}
