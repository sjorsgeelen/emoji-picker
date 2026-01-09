use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Grid, Label, Entry};
use crate::emoji::search::search;
use crate::emoji::db::EMOJI_DB;
// ...existing code...

pub struct EmojiWindow {
    window: ApplicationWindow,
}

impl EmojiWindow {
    pub fn new(app: &Application) -> Self {
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Search emoji…"));

        let grid = Grid::builder()
            .row_spacing(12)
            .column_spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        // Helper to populate grid with emojis
        let populate_grid = |grid: &Grid, emojis: &[&crate::emoji::db::Emoji]| {
            let mut child = grid.first_child();
            while let Some(widget) = child {
                let next = widget.next_sibling();
                grid.remove(&widget);
                child = next;
            }
            let columns = 8;
            for (i, emoji) in emojis.iter().enumerate() {
                let label = Label::new(Some(emoji.ch));
                label.set_css_classes(&["emoji-label"]);
                label.set_widget_name("emoji");
                grid.attach(&label, (i % columns) as i32, (i / columns) as i32, 1, 1);
            }
        };

        // Initial population
        populate_grid(&grid, &EMOJI_DB.iter().collect::<Vec<_>>());

        // Connect search field
        {
            let grid = grid.clone();
            entry.connect_changed(move |e| {
                let text = e.text();
                let results = search(text.as_str());
                populate_grid(&grid, &results);
            });
        }

        let vbox = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(6)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        vbox.append(&entry);
        vbox.append(&grid);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Emoji Picker")
            .default_width(420)
            .default_height(320)
            .child(&vbox)
            .build();

        // Add CSS for emoji font
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(
            ".emoji-label { font-family: 'Noto Color Emoji', 'Apple Color Emoji', 'Segoe UI Emoji', 'EmojiOne Color', 'Twemoji Mozilla', sans-serif; font-size: 32px; }"
        );
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("No default display found"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        Self { window }
    }
    pub fn present(&self) {
        self.window.present();
    }
}

