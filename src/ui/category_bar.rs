use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Button, Label, Align};
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
        }
        Self { button_bar }
    }
}
