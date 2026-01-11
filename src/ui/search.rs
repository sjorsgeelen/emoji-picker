use gtk4::prelude::*;
use gtk4::{Entry, Grid, ScrolledWindow};
use crate::emoji::emoji_data::Emoji;

pub struct SearchBar {
    pub entry: Entry,
    pub scrolled: ScrolledWindow,
    pub grid: Grid,
}

impl SearchBar {
    pub fn new(grid_width: i32, grid_height: i32, columns: i32, spacing: i32) -> Self {
        let entry = Entry::new();
        entry.set_size_request(grid_width, -1);
        entry.set_placeholder_text(Some("Search emoji…"));
        let grid = Grid::builder()
            .row_spacing(spacing)
            .column_spacing(spacing)
            .margin_top(0)
            .margin_bottom(0)
            .margin_start(0)
            .margin_end(0)
            .build();
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
        Self { entry, scrolled, grid }
    }
}
