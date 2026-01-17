use gtk4::prelude::*;
use gtk4::Entry;

/// SearchBar widget for emoji picker UI.
/// Handles user input and emits search query changes via callback.
pub struct SearchBar {
    entry: Entry,
    on_search: std::rc::Rc<std::cell::RefCell<Option<Box<dyn Fn(&str) + 'static>>>>,
}

impl SearchBar {
    /// Create a new SearchBar widget.
    ///
    /// Sizing should be set by the client using `search_bar.widget().set_size_request(width, height)`
    /// or by placing the widget in a GTK layout container.
    pub fn new() -> Self {
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Search emoji…"));
        let on_search: std::rc::Rc<std::cell::RefCell<Option<Box<dyn Fn(&str) + 'static>>>> = std::rc::Rc::new(std::cell::RefCell::new(None));
        let on_search_cb = on_search.clone();
        entry.connect_changed(move |e| {
            if let Some(ref cb) = *on_search_cb.borrow() {
                cb(&e.text());
            }
        });
        Self { entry, on_search }
    }

    /// Register a callback to be called when the search query changes.
    pub fn set_on_search<F: Fn(&str) + 'static>(&mut self, callback: F) {
        *self.on_search.borrow_mut() = Some(Box::new(callback));
    }

    /// Get the current search query string.
    pub fn get_query(&self) -> String {
        self.entry.text().to_string()
    }

    /// Get a reference to the underlying GTK Entry widget.
    pub fn widget(&self) -> &Entry {
        &self.entry
    }
}
