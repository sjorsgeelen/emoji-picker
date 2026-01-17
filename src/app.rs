use gtk4::prelude::*;
// ...existing code...
use gtk4::Application;

// (removed redundant mod ui;)
use crate::ui::main_window::MainWindow;

pub fn run() {
    // Initialize logging
    env_logger::init();

    // Record start time
    let _start_time = std::time::Instant::now();

    let app = Application::builder()
        .application_id("dev.sjors.EmojiPicker")
        .build();

    app.connect_activate(move |app| {
        let window = MainWindow::new(app);
        window.present();
    });

    app.run();
}