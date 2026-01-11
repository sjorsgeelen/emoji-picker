use gtk4::prelude::*;
// ...existing code...
use gtk4::Application;

use crate::ui::window::EmojiWindow;

pub fn run() {
    // Initialize logging
    env_logger::init();

    let app = Application::builder()
        .application_id("dev.sjors.EmojiPicker")
        .build();

    app.connect_activate(|app| {
        let window = EmojiWindow::new(app);
        window.present();
    });

    app.run();
}