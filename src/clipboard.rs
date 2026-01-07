use gtk4::gdk::Display;

pub fn copy(text: &str) {
    let display = Display::default().unwrap();
    let clipboard = display.clipboard();
    clipboard.set_text(text);
}