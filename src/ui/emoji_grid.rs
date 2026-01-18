// ...existing code...

impl EmojiGrid {
    /// Update the emojis displayed in the grid, clearing and rebuilding the FlowBox and label state.
    pub fn update_emojis(&mut self, emojis: &[&Emoji], grid_width: i32, grid_height: i32) {
        // Remove all children from the flowbox
        while let Some(child) = self.flowbox.first_child() {
            self.flowbox.remove(&child);
        }
        self.emoji_labels.borrow_mut().clear();
        // Add new emoji labels
        for emoji in emojis {
            let label = EmojiLabel::new(emoji.ch);
            label.set_widget_name("emoji");
            label.add_css_class("emoji-label");
            label.set_halign(gtk4::Align::Fill);
            label.set_valign(gtk4::Align::Start);
            label.set_width_request(grid_width / COLUMNS);
            label.set_height_request(grid_height / crate::ui::constants::ROWS);
            // Copy to clipboard and visual feedback on click
            let emoji_str = emoji.ch.to_string();
            let label_clone = label.clone();
            let gesture = GestureClick::new();
            gesture.connect_pressed(move |_, _, _, _| {
                clipboard::copy(&emoji_str);
                label_clone.add_css_class("copied-emoji");
                let label_inner = label_clone.clone();
                gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
                    label_inner.remove_css_class("copied-emoji");
                });
            });
            label.add_controller(gesture);
            self.flowbox.insert(&label, -1);
            self.emoji_labels.borrow_mut().push(label);
        }
        // Reset selection
        if !self.emoji_labels.borrow().is_empty() {
            *self.selected_index.borrow_mut() = Some(0);
            if let Some(label) = self.emoji_labels.borrow().get(0) {
                label.add_css_class("selected-emoji");
            }
        } else {
            *self.selected_index.borrow_mut() = None;
        }
    }
    // ...existing methods...
}
use crate::clipboard;
use crate::ui::emoji_label::EmojiLabel;
use gtk4::prelude::*;
use gtk4::{FlowBox, ScrolledWindow, GestureClick};
use crate::ui::constants::{SPACING, COLUMNS};
use std::rc::Rc;
use std::cell::RefCell;
use gtk4::glib::signal::Propagation;
use crate::emoji::emoji_data::Emoji;

pub struct EmojiGrid {
    pub scrolled: ScrolledWindow,
    pub flowbox: FlowBox,
    pub selected_index: Rc<RefCell<Option<usize>>>,
    pub emoji_labels: Rc<RefCell<Vec<gtk4::Label>>>,
    pub on_emoji_selected: Rc<RefCell<Option<Box<dyn Fn(usize) + 'static>>>>,
}

impl EmojiGrid {
    pub fn new(emojis: &[&Emoji], grid_width: i32, grid_height: i32) -> Self {
        let flowbox = FlowBox::builder()
            .row_spacing(SPACING as u32)
            .column_spacing(SPACING as u32)
            .margin_top(0)
            .margin_bottom(0)
            .margin_start(0)
            .margin_end(0)
            .min_children_per_line(COLUMNS as u32)
            .max_children_per_line(COLUMNS as u32)
            .selection_mode(gtk4::SelectionMode::None)
            .halign(gtk4::Align::Start)
            .valign(gtk4::Align::Start)
            .build();
        let emoji_labels = Rc::new(RefCell::new(Vec::new()));
        for (_, emoji) in emojis.iter().enumerate() {
            let label = EmojiLabel::new(emoji.ch);
            label.set_widget_name("emoji");
            label.add_css_class("emoji-label");
            label.set_halign(gtk4::Align::Fill);
            label.set_valign(gtk4::Align::Start);
            label.set_width_request(grid_width / COLUMNS);
            label.set_height_request(grid_height / crate::ui::constants::ROWS);
            // Copy to clipboard and visual feedback on click
            {
                let emoji = emoji.ch.to_string();
                let label_clone = label.clone();
                let gesture = GestureClick::new();
                gesture.connect_pressed(move |_, _, _, _| {
                    clipboard::copy(&emoji);
                    label_clone.add_css_class("copied-emoji");
                    let label_inner = label_clone.clone();
                    gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
                        label_inner.remove_css_class("copied-emoji");
                    });
                });
                label.add_controller(gesture);
            }
            flowbox.insert(&label, -1);
            emoji_labels.borrow_mut().push(label);
        }
        let scrolled = ScrolledWindow::builder()
            .child(&flowbox)
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

        let selected_index = Rc::new(RefCell::new(None));
        // Select the first emoji by default if any exist
        if !emoji_labels.borrow().is_empty() {
            *selected_index.borrow_mut() = Some(0);
            // Add visual selection
            if let Some(label) = emoji_labels.borrow().get(0) {
                label.add_css_class("selected-emoji");
            }
        }
        let on_emoji_selected = Rc::new(RefCell::new(None));
        let emoji_grid = Self {
            scrolled,
            flowbox,
            selected_index: selected_index.clone(),
            emoji_labels: emoji_labels.clone(),
            on_emoji_selected: on_emoji_selected.clone(),
        };

        // Keyboard navigation
        use gtk4::gdk;
        let controller = gtk4::EventControllerKey::new();
        let selected_index_clone = selected_index.clone();
        let emoji_labels_clone = emoji_labels.clone();
        let total_emojis = emoji_labels.borrow().len();
        // Focus transfer callback (to be set by window.rs)
        let focus_category_bar: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let on_emoji_selected_cb = on_emoji_selected.clone();
        controller.connect_key_pressed(move |_, keyval, _, _| {
            // Always have a valid selection
            let mut selected = selected_index_clone.borrow().unwrap_or(0);
            if total_emojis == 0 {
                return Propagation::Proceed;
            }
            match keyval {
                gdk::Key::Right => {
                    if selected + 1 < total_emojis {
                        selected += 1;
                    }
                }
                gdk::Key::Left => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                gdk::Key::Down => {
                    if selected + (COLUMNS as usize) < total_emojis {
                        selected += COLUMNS as usize;
                    }
                }
                gdk::Key::Up => {
                    if selected >= COLUMNS as usize {
                        selected -= COLUMNS as usize;
                    }
                }
                gdk::Key::Return => {
                    // Copy selected emoji to clipboard and show feedback
                    if let Some(label) = emoji_labels_clone.borrow().get(selected) {
                        let emoji = label.text().to_string();
                        clipboard::copy(&emoji);
                        label.add_css_class("copied-emoji");
                        let label_clone = label.clone();
                        gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
                            label_clone.remove_css_class("copied-emoji");
                        });
                    }
                    // Also call any registered callback
                    if let Some(ref cb) = *on_emoji_selected_cb.borrow() {
                        cb(selected);
                    }
                }
                gdk::Key::Tab if keyval == gdk::Key::ISO_Left_Tab => {
                    if let Some(ref cb) = &*focus_category_bar.borrow() {
                        cb();
                        return Propagation::Stop;
                    }
                }
                _ => {}
            }
            *selected_index_clone.borrow_mut() = Some(selected);
            for (i, label) in emoji_labels_clone.borrow().iter().enumerate() {
                if i == selected {
                    label.add_css_class("selected-emoji");
                } else {
                    label.remove_css_class("selected-emoji");
                }
            }
            Propagation::Proceed
        });
        emoji_grid.flowbox.add_controller(controller);
        // Store focus transfer callback for later wiring (handled by setter)
        emoji_grid
    }


    /// Register a callback to be called when an emoji is selected (e.g. via Return key).
    pub fn set_on_emoji_selected<F: Fn(usize) + 'static>(&mut self, callback: F) {
        *self.on_emoji_selected.borrow_mut() = Some(Box::new(callback));
    }
}
