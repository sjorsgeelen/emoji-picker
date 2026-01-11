//! Example mock app for a horizontal scrollable category bar.
//! Not production code; for layout and behavior prototyping only.

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation, ScrolledWindow, PolicyType};

fn main() {
    let app = Application::builder()
        .application_id("dev.sjors.CategoryBarMock")
        .build();

    app.connect_activate(|app| {
        // Horizontal box for category buttons
        let button_bar = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        // Add many buttons to force overflow
        for i in 0..20 {
            let emoji = ["😀", "😂", "🐶", "🍕", "🚗", "🏀", "🎵", "🌍", "💡", "📚"];
            let label = Label::new(Some(emoji[i % emoji.len()]));
            let button = Button::builder().child(&label).build();
            button_bar.append(&button);
        }
        // ScrolledWindow for horizontal scrolling
        let scrolled = ScrolledWindow::builder()
            .child(&button_bar)
            .hscrollbar_policy(PolicyType::Always)
            .min_content_width(300)
            .min_content_height(60)
            .build();
        // Main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Category Bar Mock")
            .default_width(320)
            .default_height(100)
            .child(&scrolled)
            .build();
        window.present();
    });
    app.run();
}
