use gtk4::CssProvider;

pub fn setup_css(emoji_size: i32) -> CssProvider {
    let provider = CssProvider::new();
    provider.load_from_data(&format!(
        ".emoji-label {{ font-family: 'Noto Color Emoji', 'Apple Color Emoji', 'Segoe UI Emoji', 'EmojiOne Color', 'Twemoji Mozilla', sans-serif; font-size: {}px; min-width: {}px; min-height: {}px; }}\n.tab-emoji {{ font-size: {}px; min-width: {}px; min-height: {}px; }}",
        emoji_size, emoji_size, emoji_size, emoji_size, emoji_size, emoji_size
    ));
    provider
}
