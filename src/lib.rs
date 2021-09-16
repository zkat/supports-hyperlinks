#![doc = include_str!("../README.md")]

pub use atty::Stream;

pub fn supports_hyperlinks() -> bool {
    if std::env::var("DOMTERM").is_ok() {
        // DomTerm
        return true;
    }

    if let Ok(version) = std::env::var("VTE_VERSION") {
        // VTE-based terminals above v0.50 (Gnome Terminal, Guake, ROXTerm, etc)
        if version.parse().unwrap_or(0) >= 5000 {
            return true;
        }
    }

    if let Ok(program) = std::env::var("TERM_PROGRAM") {
        if matches!(
            &program[..],
            "Hyper" | "iTerm.app" | "terminology" | "WezTerm"
        ) {
            return true;
        }
    }

    if let Ok(term) = std::env::var("TERM") {
        // Kitty
        if matches!(&term[..], "xterm-kitty") {
            return true;
        }
    }

    // Windows Terminal and Konsole
    std::env::var("WT_SESSION").is_ok() || std::env::var("KONSOLE_VERSION").is_ok()
}

pub fn on(stream: Stream) -> bool {
    atty::is(stream) && supports_hyperlinks()
}
