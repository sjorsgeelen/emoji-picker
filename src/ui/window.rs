use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use crate::emoji::emoji_data::EMOJIS;
use crate::ui::constants::*;
use crate::ui::style;
use crate::ui::category_bar::CategoryBar;
use crate::ui::emoji_grid::EmojiGrid;
use gtk4::Stack;
use log::debug;

pub struct EmojiWindow {
    window: ApplicationWindow,
}

impl EmojiWindow {
    pub fn new(app: &Application, start_time: std::time::Instant) -> Self {
    // Use constants
    let grid_rows = ROWS;
    let grid_columns = COLUMNS;
    let emoji_size = EMOJI_SIZE;
    let spacing = SPACING;
    let grid_width = (grid_columns * emoji_size) + ((grid_columns - 1) * spacing);

    // Configurable heights
    let categorybar_height = 44; // Category bar height
    let grid_height = (grid_rows * emoji_size) + ((grid_rows - 1) * spacing);
    let window_height = categorybar_height + grid_height;
    debug!("columns: {} emoji_size: {} spacing: {} grid_width: {} grid_height: {} window_height: {}", grid_columns, emoji_size, spacing, grid_width, grid_height, window_height);

    // Group emojis by category
    let mut categories: Vec<&str> = EMOJIS.iter().map(|e| e.category).collect();
    categories.sort();
    categories.dedup();

    // Stack for emoji grids
    let stack = Stack::new();

    // Add emoji grids to stack
    // Store emoji grids for focus transfer wiring
    let mut emoji_grids = Vec::new();
    for &category in categories.iter() {
        let all_emojis: Vec<_> = EMOJIS.iter().filter(|e| e.category == category).collect();
        let emoji_grid = EmojiGrid::new(&all_emojis, grid_width, grid_height);
        stack.add_named(&emoji_grid.scrolled, Some(category));
        emoji_grids.push(emoji_grid);
    }

    // Category bar
    let category_bar = CategoryBar::new(&categories, &stack, grid_width);

    // Wire up focus transfer between category bar and emoji grid
    // Assume first emoji grid is the default
    if let Some(first_grid) = emoji_grids.first() {
        // Tab from category bar moves focus to emoji grid
        let emoji_flowbox = first_grid.flowbox.clone();
        // Shift+Tab from emoji grid moves focus to category bar
        let cat_bar_widget = category_bar.button_bar.clone();
        // TODO: Actually set these callbacks in the components if exposed
        // This requires exposing a setter or callback registration in both structs
        // For now, just call grab_focus on Tab/Shift+Tab manually if needed
    }

    // Layout: category_box (bar + scrollbar), stack
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
    use gtk4::{ScrolledWindow, PolicyType};
    let category_scrolled = ScrolledWindow::builder()
        .child(&category_bar.button_bar)
        .hscrollbar_policy(PolicyType::Always)
        .min_content_width(grid_width)
        .min_content_height(60)
        .build();
    vbox.append(&category_scrolled);
    vbox.append(&stack);

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

    // Add CSS for emoji font and tab emoji size
    let provider = style::setup_css(emoji_size);
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("No default display found"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Set initial visible child for stack
    if let Some(first_cat) = categories.first() {
        stack.set_visible_child_name(first_cat);
    }

    debug!("Requested window size: width={} height={}", grid_width, window_height);
    debug!("Actual window size: width={} height={}", window.default_width(), window.default_height());
    debug!("Categorybar size: width={} height={}", category_bar.button_bar.width(), category_bar.button_bar.height());
    debug!("Stack size: width={} height={}", stack.width(), stack.height());

    window.show();

    // Log startup time
    let elapsed = start_time.elapsed();
    log::info!("Startup time: {:.2?}", elapsed);
    Self { window }
}
pub fn present(&self) {
    self.window.present();
}
}