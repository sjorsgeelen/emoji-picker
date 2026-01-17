use gtk4::prelude::*;
use gtk4::{Entry, Grid, ScrolledWindow};

pub struct SearchBar {
    pub entry: Entry,
    pub scrolled: ScrolledWindow,
    pub grid: Grid,
    pub on_search: std::rc::Rc<std::cell::RefCell<Option<Box<dyn Fn(&str) + 'static>>>>,
}

impl SearchBar {
    /// Register a callback to be called when the search query changes.
    pub fn set_on_search<F: Fn(&str) + 'static>(&mut self, callback: F) {
        *self.on_search.borrow_mut() = Some(Box::new(callback));
    }

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
        let on_search: std::rc::Rc<std::cell::RefCell<Option<Box<dyn Fn(&str) + 'static>>>> = std::rc::Rc::new(std::cell::RefCell::new(None));
        let entry_clone = entry.clone();
        let on_search_cb = on_search.clone();
        entry.connect_changed(move |e| {
            if let Some(ref cb) = *on_search_cb.borrow() {
                cb(&e.text());
            }
        });
        Self { entry, scrolled, grid, on_search }
    }
}
