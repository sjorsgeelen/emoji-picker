use gtk4::prelude::*;
use gtk4::{Grid, ScrolledWindow};
use crate::emoji::emoji_data::Emoji;

pub struct EmojiGrid {
    pub scrolled: ScrolledWindow,
    pub grid: Grid,
}

impl EmojiGrid {
    pub fn new(emojis: &[&Emoji], columns: i32, emoji_size: i32, spacing: i32, grid_width: i32, grid_height: i32) -> Self {
        let grid = Grid::builder()
            .row_spacing(spacing)
            .column_spacing(spacing)
            .margin_top(0)
            .margin_bottom(0)
            .margin_start(0)
            .margin_end(0)
            .build();
        for (i, emoji) in emojis.iter().enumerate() {
            let label = gtk4::Label::new(Some(emoji.ch));
            label.set_css_classes(&["emoji-label"]);
            label.set_widget_name("emoji");
            grid.attach(&label, (i % columns as usize) as i32, (i / columns as usize) as i32, 1, 1);
        }
        let scrolled = ScrolledWindow::builder()
            .child(&grid)
            .min_content_height(grid_height)
            .max_content_height(grid_height)
            .min_content_width(grid_width)
            .max_content_width(grid_width)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .margin_top(0)
            .margin_bottom(0)
            .margin_start(0)
            .margin_end(0)
            .build();
        scrolled.set_size_request(grid_width, grid_height);
        Self { scrolled, grid }
    }
}
