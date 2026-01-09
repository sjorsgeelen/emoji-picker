use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Grid, Label, Entry};
use crate::emoji::search::search;
use crate::emoji::emoji_data::EMOJIS;
use crate::emoji::emoji_data::Emoji;
use gtk4::{Stack, ScrolledWindow, Orientation, Button, Box as GtkBox};

pub struct EmojiWindow {
    window: ApplicationWindow,
}

impl EmojiWindow {
    pub fn new(app: &Application) -> Self {
        // Custom tab bar and Stack
        let columns = 5;
        let emoji_size = 32; // px, matches font-size in CSS
        let spacing = 12; // matches row/column spacing
        let margin = 0; // no margin for grid, so window fits grid exactly
        let grid_width = (columns * emoji_size) + ((columns - 1) * spacing);

        // --- Search results grid (hidden by default) ---
        let search_grid = Grid::builder()
            .row_spacing(spacing)
            .column_spacing(spacing)
            .margin_top(0)
            .margin_bottom(0)
            .margin_start(0)
            .margin_end(0)
            .build();
        let search_scrolled = ScrolledWindow::builder()
            .child(&search_grid)
            .min_content_height(200)
            .min_content_width(grid_width)
            .max_content_width(grid_width)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .margin_top(0)
            .margin_bottom(0)
            .margin_start(0)
            .margin_end(0)
            .build();
        search_scrolled.set_visible(false);
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Search emoji…"));

        // Group emojis by category
        let mut categories: Vec<&str> = EMOJIS.iter().map(|e| e.category).collect();
        categories.sort();
        categories.dedup();

        // Custom tab bar and Stack
        let columns = 5;
        let emoji_size = 32; // px, matches font-size in CSS
        let spacing = 12; // matches row/column spacing
        let margin = 0; // no margin for grid, so window fits grid exactly
        let grid_width = (columns * emoji_size) + ((columns - 1) * spacing);

        // Stack for emoji grids
        let stack = Stack::new();
        let mut grid_vec = Vec::new();
        let mut loaded_counts = Vec::new();

        // Horizontal Box for tab bar
        let tab_bar = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .build();

        // ScrolledWindow for tab bar (horizontal only)
        let tab_scroller = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Never)
            .child(&tab_bar)
            .min_content_width(grid_width as i32)
            .max_content_height(emoji_size + 8)
            .build();

        for (cat_idx, &category) in categories.iter().enumerate() {
            let grid = Grid::builder()
                .row_spacing(spacing)
                .column_spacing(spacing)
                .margin_top(0)
                .margin_bottom(0)
                .margin_start(0)
                .margin_end(0)
                .build();

            let all_emojis: Vec<&Emoji> = EMOJIS.iter().filter(|e| e.category == category).collect();
            let loaded = 40;
            // Initial load
            for (i, emoji) in all_emojis.iter().take(loaded).enumerate() {
                let label = Label::new(Some(emoji.ch));
                label.set_css_classes(&["emoji-label"]);
                label.set_widget_name("emoji");
                grid.attach(&label, (i % columns as usize) as i32, (i / columns as usize) as i32, 1, 1);
            }

            let scrolled = ScrolledWindow::builder()
                .child(&grid)
                .min_content_height(200)
                .min_content_width(grid_width)
                .max_content_width(grid_width)
                .hscrollbar_policy(gtk4::PolicyType::Never)
                .margin_top(0)
                .margin_bottom(0)
                .margin_start(0)
                .margin_end(0)
                .build();
            grid.set_hexpand(false);

            // Lazy load more emojis on scroll (logic will be updated in next step)
            let adj = scrolled.vadjustment();
            let grid_clone = grid.clone();
            let all_emojis_clone = all_emojis.clone();
            let columns = columns;
            adj.connect_value_changed(move |adj: &gtk4::Adjustment| {
                let value = adj.value();
                let upper = adj.upper();
                let page_size = adj.page_size();
                if value + page_size >= upper - 1.0 {
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
                        grid_clone.attach(&label, (i % columns as usize) as i32, (i / columns as usize) as i32, 1, 1);
                        label.show();
                    }
                }
            });

            // Add grid to stack
            stack.add_named(&scrolled, Some(category));
            grid_vec.push(grid);
            loaded_counts.push(loaded);

            // Tab bar button (emoji icon)
            let tab_emoji = all_emojis.first().map(|e| e.ch).unwrap_or("?");
            let tab_label = Label::new(Some(tab_emoji));
            tab_label.set_css_classes(&["emoji-label", "tab-emoji"]);
            let button = Button::builder()
                .child(&tab_label)
                .build();
            let stack_clone = stack.clone();
            let cat_name = category.to_string();
            button.connect_clicked(move |_| {
                stack_clone.set_visible_child_name(&cat_name);
            });
            tab_bar.append(&button);
        }



        // Layout: entry, tab bar (scrolled), stack
        let vbox = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        vbox.append(&entry);
        vbox.append(&tab_scroller);
        vbox.append(&stack);
        vbox.append(&search_scrolled);
        // --- Search entry logic ---
        let stack_clone = stack.clone();
        let tab_scroller_clone = tab_scroller.clone();
        let search_grid_clone = search_grid.clone();
        let search_scrolled_clone = search_scrolled.clone();
        entry.connect_changed(move |entry| {
            let text = entry.text().to_string();
            let is_searching = !text.trim().is_empty();
            if is_searching {
                // Hide tabs and stack, show search grid
                tab_scroller_clone.set_visible(false);
                stack_clone.set_visible(false);
                search_scrolled_clone.set_visible(true);
                // Clear previous search results
                let mut child = search_grid_clone.first_child();
                while let Some(widget) = child {
                    let next = widget.next_sibling();
                    search_grid_clone.remove(&widget);
                    child = next;
                }
                // Add new search results
                let results = crate::emoji::search::search(&text);
                for (i, emoji) in results.iter().enumerate() {
                    let label = Label::new(Some(emoji.ch));
                    label.set_css_classes(&["emoji-label"]);
                    label.set_widget_name("emoji");
                    search_grid_clone.attach(&label, (i % columns as usize) as i32, (i / columns as usize) as i32, 1, 1);
                }
            } else {
                // Restore tabs and stack
                tab_scroller_clone.set_visible(true);
                stack_clone.set_visible(true);
                search_scrolled_clone.set_visible(false);
            }
        });

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Emoji Picker")
            .default_width(grid_width)
            .default_height(320)
            .resizable(false)
            .child(&vbox)
            .build();

        // Add CSS for emoji font and tab emoji size
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&format!(
            ".emoji-label {{ font-family: 'Noto Color Emoji', 'Apple Color Emoji', 'Segoe UI Emoji', 'EmojiOne Color', 'Twemoji Mozilla', sans-serif; font-size: {}px; min-width: {}px; min-height: {}px; }}\n.tab-emoji {{ font-size: {}px; min-width: {}px; min-height: {}px; }}",
            emoji_size, emoji_size, emoji_size, emoji_size, emoji_size, emoji_size
        ));
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("No default display found"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Set initial visible child for stack
        if let Some(first_cat) = categories.first() {
            stack.set_visible_child_name(first_cat);
        }

        Self { window }
    }
    pub fn present(&self) {
        self.window.present();
    }
}

