use gtk4::CssProvider;
use std::fs;

/// Loads and applies the emoji picker CSS from a static file.
///
/// All dynamic sizing (font-size, min-width, min-height) should be set directly on widgets in Rust.
pub fn setup_css() -> CssProvider {
    use std::env;
    let provider = CssProvider::new();
    let mut tried = Vec::new();
    let candidates = [
        "data/style.css", // dev
        "/app/share/emoji-picker/style.css", // Flatpak
        // XDG data dir (Linux best practice)
        &format!("{}/emoji-picker/style.css", env::var("XDG_DATA_HOME").unwrap_or_else(|_| String::from("~/.local/share"))),
    ];
    let css = candidates.iter().find_map(|path| {
        tried.push(path.to_string());
        fs::read_to_string(path).ok()
    });
    match css {
        Some(css) => {
            provider.load_from_data(&css);
            provider
        }
        None => {
            panic!("Failed to read CSS file. Tried: {}", tried.join(", "));
        }
    }
}
