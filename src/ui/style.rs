use gtk4::CssProvider;
use std::fs;
use std::path::Path;

/// Loads and applies the emoji picker CSS from a static file.
///
/// All dynamic sizing (font-size, min-width, min-height) should be set directly on widgets in Rust.
pub fn setup_css() -> CssProvider {
    let provider = CssProvider::new();
    let css_path = Path::new("data/style.css");
    let css = fs::read_to_string(css_path)
        .expect("Failed to read CSS file at data/style.css");
    provider.load_from_data(&css);
    provider
}
