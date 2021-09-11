pub fn supports_hyperlinks() -> bool {
    if std::env::var("DOMTERM").is_ok() {
        // DomTerm
        true
    } else if let Ok(version) = std::env::var("VTE_VERSION") {
        // VTE-based terminals above v0.50 (Gnome Terminal, Guake, ROXTerm, etc)
        version.parse().unwrap_or(0) >= 5000
    } else if let Ok(program) = std::env::var("TERM_PROGRAM") {
        matches!(
            &program[..],
            "Hyper" | "iTerm.app" | "terminology" | "WezTerm"
        )
    } else if let Ok(term) = std::env::var("TERM") {
        // Kitty
        matches!(&term[..], "xterm-kitty")
    } else {
        // Windows Terminal
        std::env::var("WT_SESSION").is_ok() ||
            // Konsole
            std::env::var("KONSOLE_VERSION").is_ok()
    }
}
