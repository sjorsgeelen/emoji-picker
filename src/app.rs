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

    use std::cell::RefCell;
    use std::rc::Rc;
    let window_ref: Rc<RefCell<Option<MainWindow>>> = Rc::new(RefCell::new(None));
    let app = Application::builder()
        .application_id("nl.dibitat.emoji_picker")
        .build();

    let window_ref_clone = window_ref.clone();
    app.connect_activate(move |app| {
        let mut win = window_ref_clone.borrow_mut();
        if win.is_none() {
            *win = Some(MainWindow::new(app));
        }
        if let Some(ref window) = *win {
            window.present();
        }
    });

    app.run();
}