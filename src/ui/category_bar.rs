use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Button, Label, Align};
use std::rc::Rc;
use std::cell::RefCell;
use gtk4::glib::signal::Propagation;
use gtk4::gdk;
/// Represents the category selection bar in the emoji picker UI.
///
/// Contains the emoji tab buttons as a horizontal box.
///
/// ## Scrolling
/// This widget does not implement scrolling itself. If you want the category bar to be horizontally scrollable,
/// wrap `CategoryBar::button_bar` in a `gtk4::ScrolledWindow` with `hscrollbar_policy(PolicyType::Always)`.
/// This allows clients to control layout and scrolling behavior as needed.
pub struct CategoryBar {
    /// The horizontal box containing the emoji tab buttons.
    pub button_bar: GtkBox,
    pub selected_index: Rc<RefCell<Option<usize>>>,
    pub buttons: Rc<RefCell<Vec<Button>>>,
    pub on_category_selected: Rc<RefCell<Option<Box<dyn Fn(usize) + 'static>>>>,
}

impl CategoryBar {
    /// Creates a new CategoryBar with emoji tab buttons for each category.
    ///
    /// # Arguments
    /// * `categories` - List of category names.
    /// * `stack` - The Stack widget to switch visible emoji grids.
    /// * `grid_width` - The width to constrain the tab bar and viewport.
    ///
    /// The tab buttons use the first emoji of each category as their label.
    pub fn new(categories: &[&str], stack: &gtk4::Stack, _grid_width: i32) -> Self {
        let button_bar = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .hexpand(false)
            .halign(Align::Start)
            .build();
        use crate::emoji::emoji_data::EMOJIS;
        let buttons = Rc::new(RefCell::new(Vec::new()));
        for &category in categories.iter() {
            // Find the first emoji in this category
            let tab_emoji = EMOJIS.iter().find(|e| e.category == category).map(|e| e.ch).unwrap_or("?");
            let tab_label = Label::new(Some(tab_emoji));
            tab_label.set_css_classes(&["emoji-label", "tab-emoji"]);
            let button = Button::builder().child(&tab_label).build();
            let stack_clone = stack.clone();
            let cat_name = category.to_string();
            button.connect_clicked(move |_| {
                stack_clone.set_visible_child_name(&cat_name);
            });
            button_bar.append(&button);
            buttons.borrow_mut().push(button);
        }
        let selected_index = Rc::new(RefCell::new(None));
        let on_category_selected = Rc::new(RefCell::new(None));
        let category_bar = Self {
            button_bar,
            selected_index: selected_index.clone(),
            buttons: buttons.clone(),
            on_category_selected: on_category_selected.clone(),
        };

        let controller = gtk4::EventControllerKey::new();
        let selected_index_clone = selected_index.clone();
        let buttons_clone = buttons.clone();
        // Focus transfer callback (to be set by window.rs)
        let focus_emoji_grid: Option<Box<dyn Fn()>> = None;
        controller.connect_key_pressed(move |_, keyval, _, _| {
            let total = buttons_clone.borrow().len();
            let mut selected = selected_index_clone.borrow().unwrap_or(0);
            match keyval {
                gdk::Key::Right => {
                    if selected + 1 < total {
                        selected += 1;
                    }
                }
                gdk::Key::Left => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                gdk::Key::Return => {
                    // Activate category
                    if let Some(button) = buttons_clone.borrow().get(selected) {
                        button.emit_clicked();
                    }
                }
                gdk::Key::Tab => {
                    // Move focus to emoji grid
                    if let Some(ref cb) = focus_emoji_grid {
                        cb();
                        return Propagation::Stop;
                    }
                }
                _ => {}
            }
            *selected_index_clone.borrow_mut() = Some(selected);
            // Update visual selection (e.g., CSS class)
            for (i, button) in buttons_clone.borrow().iter().enumerate() {
                if i == selected {
                    button.add_css_class("selected-category");
                } else {
                    button.remove_css_class("selected-category");
                }
            }
            Propagation::Proceed
        });
        category_bar.button_bar.add_controller(controller);

        let on_category_selected_cb = on_category_selected.clone();
        for (i, button) in buttons.borrow().iter().enumerate() {
            let idx = i;
            let on_category_selected_cb = on_category_selected_cb.clone();
            button.connect_clicked(move |_| {
                if let Some(ref cb) = *on_category_selected_cb.borrow() {
                    cb(idx);
                }
            });
        }

        category_bar
    }
    /// Register a callback to be called when a category is selected (button clicked).
    pub fn set_on_category_selected<F: Fn(usize) + 'static>(&mut self, callback: F) {
        *self.on_category_selected.borrow_mut() = Some(Box::new(callback));
    }
}
