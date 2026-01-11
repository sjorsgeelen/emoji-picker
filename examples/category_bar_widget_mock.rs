//! Example mock app for the CategoryBar widget implementation.
//! Not production code; for layout and behavior prototyping only.

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Stack};
use emoji_picker::ui::category_bar::CategoryBar;
use emoji_picker::ui::constants::*;

// Note: Run with `cargo run --example category_bar_widget_mock`

fn main() {
    let app = Application::builder()
        .application_id("dev.sjors.CategoryBarWidgetMock")
        .build();

    app.connect_activate(|app| {
        // Example categories
        let categories = vec![
            "Smileys & Emotion", "People & Body", "Animals & Nature",
            "Food & Drink", "Travel & Places", "Activities", "Objects",
            "Symbols", "Flags"
        ];
        let grid_width = (COLUMNS * EMOJI_SIZE) + ((COLUMNS - 1) * SPACING);
        let stack = Stack::new();
        let category_bar = CategoryBar::new(&categories, &stack, grid_width);

        use gtk4::{ScrolledWindow, PolicyType};
        let scrolled = ScrolledWindow::builder()
            .child(&category_bar.button_bar)
            .hscrollbar_policy(PolicyType::Always)
            .min_content_width(grid_width)
            .min_content_height(60)
            .build();
        let window = ApplicationWindow::builder()
            .application(app)
            .title("CategoryBar Widget Mock")
            .default_width(grid_width)
            .default_height(100)
            .child(&scrolled)
            .build();
        window.present();
    });
    app.run();
}
