//! Main window for the emoji picker, using EmojiPickerController for all UI logic.

use crate::emoji::emoji_data::EMOJIS;
use crate::ui::category_bar::CategoryBar;
use crate::ui::constants::*;
use crate::ui::emoji_grid::EmojiGrid;
use crate::ui::style;
use gtk4::prelude::*;
use gtk4::Stack;
use gtk4::{Application, ApplicationWindow};

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        let grid_rows = ROWS;
        let grid_columns = COLUMNS;
        let emoji_size = EMOJI_SIZE;
        let spacing = SPACING;
        let grid_width = (grid_columns * emoji_size) + ((grid_columns - 1) * spacing);

        let categorybar_height = 44;
        let grid_height = (grid_rows * emoji_size) + ((grid_rows - 1) * spacing);
        let window_height = categorybar_height + grid_height;

        let mut categories: Vec<&str> = EMOJIS.iter().map(|e| e.category).collect();
        categories.sort();
        categories.dedup();

        let stack = Stack::new();

        let mut emoji_grids = Vec::new();
        for &category in categories.iter() {
            let all_emojis: Vec<_> = EMOJIS.iter().filter(|e| e.category == category).collect();
            let emoji_grid = EmojiGrid::new(&all_emojis, grid_width, grid_height);
            stack.add_named(&emoji_grid.scrolled, Some(category));
            emoji_grids.push(emoji_grid);
        }
        let search_results_grid = EmojiGrid::new(&[], grid_width, grid_height);
        stack.add_named(&search_results_grid.scrolled, Some("__search__"));

        let category_bar = CategoryBar::new(&categories, &stack, grid_width);

        let vbox = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(6)
            .margin_top(0)
            .margin_bottom(0)
            .margin_start(0)
            .margin_end(0)
            .hexpand(false)
            .halign(gtk4::Align::Start)
            .build();

        let mut search_bar = crate::ui::search::SearchBar::new();
        let category_scrolled = gtk4::ScrolledWindow::builder()
            .child(&category_bar.button_bar)
            .hscrollbar_policy(gtk4::PolicyType::Always)
            .min_content_width(grid_width)
            .min_content_height(60)
            .build();

        vbox.append(search_bar.widget());
        vbox.append(&category_scrolled);
        vbox.append(&stack);

        let all_emojis: Vec<_> = EMOJIS.iter().cloned().collect();
        let controller = std::rc::Rc::new(std::cell::RefCell::new(
            crate::ui::app_controller::EmojiPickerController::new(all_emojis),
        ));
        let category_scrolled_clone = category_scrolled.clone();
        let stack_clone = stack.clone();
        let search_results_grid_clone = search_results_grid;
        let categories_for_closure = categories.clone();
        let _controller_for_closure = controller.clone();
        // Register UI update listener
        controller.borrow_mut().add_listener({
            let category_scrolled_clone = category_scrolled_clone.clone();
            let stack_clone = stack_clone.clone();
            let search_results_grid_clone = search_results_grid_clone;
            let categories_for_closure = categories_for_closure.clone();
            move |mode, filtered_emojis: &[crate::emoji::emoji_data::Emoji]| {
                if mode == crate::ui::app_controller::PickerMode::Search {
                    category_scrolled_clone.set_visible(false);
                    log::info!("UI listener: displaying {} emojis in search results grid", filtered_emojis.len());
                    
                    while let Some(child) = search_results_grid_clone.flowbox.first_child() {
                        search_results_grid_clone.flowbox.remove(&child);
                    }
                    for emoji in filtered_emojis {
                        let label = crate::ui::emoji_label::EmojiLabel::new(emoji.ch);
                        search_results_grid_clone.flowbox.insert(&label, -1);
                    }
                    stack_clone.set_visible_child_name("__search__");
                } else {
                    category_scrolled_clone.set_visible(true);
                    if let Some(first_cat) = categories_for_closure.first() {
                        stack_clone.set_visible_child_name(first_cat);
                    }
                }
            }
        });
        // Only trigger controller logic on search
        search_bar.set_on_search(move |query| {
            log::info!("SearchBar event: query='{}'", query);
            controller.borrow_mut().handle_search(query);
        });

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Emoji Picker")
            .default_width(grid_width)
            .default_height(window_height)
            .resizable(false)
            .child(&vbox)
            .build();
        window.set_size_request(grid_width, window_height);
        window.set_resizable(false);

        let provider = style::setup_css();
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("No default display found"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        if let Some(first_cat) = categories.first() {
            stack.set_visible_child_name(first_cat);
        }

        window.show();

        Self { window }
    }
    pub fn present(&self) {
        self.window.present();
    }
}
