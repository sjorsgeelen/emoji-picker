use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Grid, Label, Entry};
use crate::emoji::search::search;
use crate::emoji::emoji_data::EMOJIS;
use crate::emoji::emoji_data::Emoji;
use gtk4::{Notebook, ScrolledWindow, Orientation};

pub struct EmojiWindow {
    window: ApplicationWindow,
}

impl EmojiWindow {
    pub fn new(app: &Application) -> Self {
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Search emoji…"));

        // Group emojis by category
        let mut categories: Vec<&str> = EMOJIS.iter().map(|e| e.category).collect();
        categories.sort();
        categories.dedup();

        let notebook = Notebook::new();
        let mut grid_vec = Vec::new();
        let mut loaded_counts = Vec::new();

        for &category in &categories {
            let grid = Grid::builder()
                .row_spacing(12)
                .column_spacing(12)
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            let all_emojis: Vec<&Emoji> = EMOJIS.iter().filter(|e| e.category == category).collect();
            let loaded = 40;
            let columns = 8;
            // Initial load
            for (i, emoji) in all_emojis.iter().take(loaded).enumerate() {
                let label = Label::new(Some(emoji.ch));
                label.set_css_classes(&["emoji-label"]);
                label.set_widget_name("emoji");
                grid.attach(&label, (i % columns) as i32, (i / columns) as i32, 1, 1);
            }

            let scrolled = ScrolledWindow::builder()
                .child(&grid)
                .min_content_height(200)
                .build();

            // Lazy load more emojis on scroll
            let adj = scrolled.vadjustment();
            let grid_clone = grid.clone();
            let all_emojis_clone = all_emojis.clone();
            let columns = columns;
            adj.connect_value_changed(move |adj: &gtk4::Adjustment| {
                let value = adj.value();
                let upper = adj.upper();
                let page_size = adj.page_size();
                // If scrolled to bottom, load more
                if value + page_size >= upper - 1.0 {
                    // Count children in grid using first_child/next_sibling
                    let mut current_count = 0;
                    let mut child = grid_clone.first_child();
                    while let Some(widget) = child {
                        current_count += 1;
                        child = widget.next_sibling();
                    }
                    let to_load = 40;
                    for (i, emoji) in all_emojis_clone.iter().enumerate().skip(current_count).take(to_load) {
                        let label = Label::new(Some(emoji.ch));
                        label.set_css_classes(&["emoji-label"]);
                        label.set_widget_name("emoji");
                        grid_clone.attach(&label, (i % columns) as i32, (i / columns) as i32, 1, 1);
                        label.show();
                    }
                }
            });

            // Use the first emoji of the category as the tab label
            let tab_emoji = all_emojis.first().map(|e| e.ch).unwrap_or("?");
            let tab_label = Label::new(Some(tab_emoji));
            tab_label.set_css_classes(&["emoji-label"]);
            notebook.append_page(&scrolled, Some(&tab_label));
            grid_vec.push(grid);
            loaded_counts.push(loaded);
        }

        // TODO: Connect search field to filter emojis in the active tab only

        let vbox = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        vbox.append(&entry);
        vbox.append(&notebook);

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

