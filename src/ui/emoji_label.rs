use gtk4::prelude::*;
use gtk4::{Label, Align};
use gtk4::pango;
use crate::ui::constants::EMOJI_SIZE;

/// Struct to encapsulate emoji label configuration and creation.
pub struct EmojiLabel;

impl EmojiLabel {
    /// Create a new emoji label with consistent styling and sizing.
    pub fn new(text: &str) -> Label {
        let label = Label::new(Some(text));
        label.add_css_class("emoji-label");
        label.set_halign(Align::Center);
        label.set_valign(Align::Center);
        label.set_width_request(EMOJI_SIZE);
        label.set_height_request(EMOJI_SIZE);

        // Set font size using Pango (in points * PANGO_SCALE)
        let attr_list = pango::AttrList::new();
        let font_size_attr = pango::AttrSize::new((EMOJI_SIZE * pango::SCALE) as i32);
        attr_list.insert(font_size_attr.upcast());
        label.set_attributes(Some(&attr_list));

        label
    }
}
