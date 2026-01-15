use gtk4::prelude::*;
// ...existing code...
use gtk4::Application;

use crate::ui::window::EmojiWindow;

pub fn run() {
    // Initialize logging
    env_logger::init();

    // Record start time
    let start_time = std::time::Instant::now();

    let app = Application::builder()
        .application_id("dev.sjors.EmojiPicker")
        .build();

    app.connect_activate(move |app| {
        let window = EmojiWindow::new(app, start_time);
        window.present();
    });

    app.run();
}