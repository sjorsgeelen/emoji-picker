//! Main window for the emoji picker, using EmojiPickerController for all UI logic.

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation};
use crate::ui::app_controller::EmojiPickerController;
use crate::ui::search::SearchBar;
use crate::ui::category_bar::CategoryBar;
use crate::ui::emoji_grid::EmojiGrid;
use crate::emoji::emoji_data::EMOJIS;

pub struct MainWindow {
    pub window: ApplicationWindow,
    pub controller: std::rc::Rc<std::cell::RefCell<EmojiPickerController>>,
    pub search_bar: SearchBar,
    pub category_bar: CategoryBar,
    pub emoji_grid: EmojiGrid,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        // Layout constants
        let grid_rows = crate::ui::constants::ROWS;
        let grid_columns = crate::ui::constants::COLUMNS;
        let emoji_size = crate::ui::constants::EMOJI_SIZE;
        let spacing = crate::ui::constants::SPACING;
        let grid_width = (grid_columns * emoji_size) + ((grid_columns - 1) * spacing);
        let grid_height = (grid_rows * emoji_size) + ((grid_rows - 1) * spacing);

        // Instantiate UI components
        let search_bar = SearchBar::new(grid_width, 32, grid_columns, spacing);
        // Derive categories from emoji data
        let mut categories: Vec<&str> = EMOJIS.iter().map(|e| e.category).collect();
        categories.sort();
        categories.dedup();
        let category_bar = CategoryBar::new(&categories, &gtk4::Stack::new(), grid_width);
        let emoji_refs: Vec<&crate::emoji::emoji_data::Emoji> = EMOJIS.iter().collect();
        let emoji_grid = EmojiGrid::new(&emoji_refs, grid_width, grid_height);
        let all_emojis = EMOJIS.to_vec();

        // Create controller (pure, no GTK widgets)
        let controller = std::rc::Rc::new(std::cell::RefCell::new(EmojiPickerController::new(all_emojis)));

        // Layout
        let vbox = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .margin_top(0)
            .margin_bottom(0)
            .margin_start(0)
            .margin_end(0)
            .hexpand(false)
            .halign(gtk4::Align::Start)
            .build();
        vbox.append(&search_bar.entry);
        vbox.append(&category_bar.button_bar);
        vbox.append(&emoji_grid.scrolled);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Emoji Picker")
            .default_width(grid_width)
            .default_height(grid_height + 44 + 32)
            .resizable(false)
            .child(&vbox)
            .build();
        window.set_size_request(grid_width, grid_height + 44 + 32);
        window.set_resizable(false);

        // Wire up widget events to controller handlers
        {
            let controller_clone = controller.clone();
            let search_bar_ref = search_bar.on_search.clone();
            *search_bar_ref.borrow_mut() = Some(Box::new(move |query: &str| {
                controller_clone.borrow_mut().handle_search(query);
                // TODO: update emoji_grid_ref with filtered_emojis
            }));
            // TODO: Connect category bar and emoji grid events similarly
        }

        Self { window, controller, search_bar, category_bar, emoji_grid }
    }
    pub fn present(&self) {
        self.window.present();
    }
}
