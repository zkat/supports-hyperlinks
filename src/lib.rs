#![doc = include_str!("../README.md")]

/// possible stream sources
#[derive(Clone, Copy, Debug)]
pub enum Stream {
    Stdout,
    Stderr,
}

/// Returns true if the current terminal, detected through various environment
/// variables, is known to support hyperlink rendering.
pub fn supports_hyperlinks() -> bool {
    // Hyperlinks can be forced through this env var.
    if let Some(arg) = std::env::var_os("FORCE_HYPERLINK") {
        return arg != "0";
    }

    supports_hyperlinks_without_force_check()
}

#[inline]
fn supports_hyperlinks_without_force_check() -> bool {
    if std::env::var_os("DOMTERM").is_some() {
        // DomTerm
        return true;
    }

    if supported_vte() {
        return true;
    }

    if let Some(program) = std::env::var_os("TERM_PROGRAM") {
        if let Some("Hyper" | "iTerm.app" | "terminology" | "WezTerm" | "vscode" | "ghostty") =
            program.to_str()
        {
            return true;
        }
    }

    if let Some(term) = std::env::var_os("TERM") {
        if let Some("xterm-kitty" | "alacritty" | "alacritty-direct") = term.to_str() {
            return true;
        }
    }

    if let Some(term) = std::env::var_os("COLORTERM") {
        if term == "xfce4-terminal" {
            return true;
        }
    }

    // Windows Terminal and Konsole
    std::env::var_os("WT_SESSION").is_some() || std::env::var_os("KONSOLE_VERSION").is_some()
}

// VTE-based terminals above v0.50 (Gnome Terminal, Guake, ROXTerm, etc).
// The version must be >= 5000.
#[inline]
fn supported_vte() -> bool {
    let Some(version) = std::env::var_os("VTE_VERSION") else {
        return false;
    };

    let Some(version) = version.to_str() else {
        return false;
    };

    // Instead of parsing the string to an unsigned integer and then compare with 5000,
    // we will verify the digits directly.
    let version = version.as_bytes();
    // At least 4 digits where the first isn't 0.
    let [c0 @ b'1'..=b'9', b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', rest @ ..] = version else {
        return false;
    };

    if rest.is_empty() {
        // Only 4 digits. The first one must be >= 5 for the version to be >= 5000.
        return *c0 >= b'5';
    }

    // More than 4 digits. If all chars in `rest` are also digits, then the value must be >= 10000.
    rest.iter().all(|c| c.is_ascii_digit())
}

#[inline]
fn is_a_tty(stream: Stream) -> bool {
    use std::io::IsTerminal;
    match stream {
        Stream::Stdout => std::io::stdout().is_terminal(),
        Stream::Stderr => std::io::stderr().is_terminal(),
    }
}

/// Returns true if `stream` is a TTY, and the current terminal
/// [supports_hyperlinks].
pub fn on(stream: Stream) -> bool {
    // Hyperlinks can be forced through this env var.
    if let Some(arg) = std::env::var_os("FORCE_HYPERLINK") {
        return arg != "0";
    }

    is_a_tty(stream) && supports_hyperlinks_without_force_check()
}
