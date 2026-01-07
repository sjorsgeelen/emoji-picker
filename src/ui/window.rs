use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Entry, ListBox, ListBoxRow, Label};
use crate::emoji::search::search;

pub struct EmojiWindow {
    window: ApplicationWindow,
}

impl EmojiWindow {
    pub fn new(app: &Application) -> Self {
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Search emoji…"));

        let list = ListBox::new();
        list.set_selection_mode(gtk4::SelectionMode::Single);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Emoji Picker")
            .default_width(420)
            .default_height(320)
            .child(
                &gtk4::Box::builder()
                    .orientation(gtk4::Orientation::Vertical)
                    .spacing(6)
                    .margin_top(12)
                    .margin_bottom(12)
                    .margin_start(12)
                    .margin_end(12)
                    .append(&entry)
                    .append(&list)
                    .build(),
            )
            .build();

        entry.connect_changed(move |e| {
            list.foreach(|child| list.remove(child));

            let text = e.text();
            for emoji in search(text.as_str()) {
                let row = ListBoxRow::new();
                row.set_child(Some(
                    &Label::new(Some(&format!("{}  {}", emoji.ch, emoji.name)))
                ));
                list.append(&row);
            }
        });

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}