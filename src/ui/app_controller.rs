//! Central controller and state for the emoji picker UI.
//! Owns all UI components and coordinates their interaction via callbacks/signals.

use crate::emoji::emoji_data::Emoji;

/// The current mode of the picker UI.
/// The current mode of the picker UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickerMode {
    /// Default mode: show categories and category grid.
    Browse,
    /// Search mode: show search results grid, hide/de-emphasize category bar.
    Search,
}

/// Centralized state and controller for the emoji picker UI, pure and testable.
pub struct EmojiPickerController {
    pub mode: PickerMode,
    pub search_query: String,
    pub all_emojis: Vec<Emoji>,
    pub filtered_emojis: Vec<Emoji>,
    listeners: Vec<Box<dyn Fn(PickerMode, &[Emoji])>>, // Observer pattern
}

impl EmojiPickerController {
    /// Register a callback to be called when the controller state changes.
    pub fn add_listener<F: Fn(PickerMode, &[Emoji]) + 'static>(&mut self, f: F) {
        self.listeners.push(Box::new(f));
    }

    fn notify_listeners(&self) {
        for cb in &self.listeners {
            cb(self.mode, &self.filtered_emojis);
        }
    }
    /// Returns true if the category bar should be shown (i.e., in Browse mode).
    pub fn show_category_bar(&self) -> bool {
        matches!(self.mode, PickerMode::Browse)
    }
    pub fn new(all_emojis: Vec<Emoji>) -> Self {
        Self {
            mode: PickerMode::Browse,
            search_query: String::new(),
            all_emojis,
            filtered_emojis: Vec::new(),
            listeners: Vec::new(), // Initialize listeners
        }
    }

    /// Handle a search query update. Switches mode and updates filtered results.
    pub fn handle_search(&mut self, query: &str) {
        log::info!(
            "handle_search called with query: '{}', current mode: {:?}",
            query,
            self.mode
        );
        self.search_query = query.to_string();
        if query.is_empty() {
            log::info!("Switching to Browse mode (empty query)");
            self.mode = PickerMode::Browse;
            self.filtered_emojis.clear();
        } else {
            log::info!("Switching to Search mode (query: '{}')", query);
            self.mode = PickerMode::Search;
            let q = query.to_lowercase();
            let filtered: Vec<_> = self
                .all_emojis
                .iter()
                .filter(|e| {
                    e.name_en.to_lowercase().contains(&q)
                        || e.keywords_en.iter().any(|k| k.to_lowercase().contains(&q))
                        || e.name_nl.to_lowercase().contains(&q)
                        || e.keywords_nl.iter().any(|k| k.to_lowercase().contains(&q))
                        || e.ch.contains(&q)
                })
                .cloned()
                .collect();
            self.filtered_emojis = filtered;
        }
        log::info!(
            "After search, mode is now: {:?}, filtered_emojis: {}",
            self.mode,
            self.filtered_emojis.len()
        );
        log::info!("Calling notify_listeners ({} listeners)", self.listeners.len());
        self.notify_listeners(); // Notify listeners after handling search
    }

    pub fn handle_category_selected(&mut self, _category_idx: usize) {
        // In real UI, would update grid
    }

    pub fn handle_emoji_selected(&mut self, _emoji_idx: usize) {
        // In real UI, would handle emoji selection
    }
}

#[cfg(test)]
#[test]
fn test_show_category_bar_logic() {
    let emojis = vec![crate::emoji::emoji_data::Emoji {
        ch: "😀",
        name_en: "grinning face",
        keywords_en: &[],
        name_nl: "grijnzend gezicht",
        keywords_nl: &[],
        category: "Smileys & Emotion",
        skin_tone_variants: None,
    }];
    let mut controller = EmojiPickerController::new(emojis.clone());
    // Initially in Browse mode
    assert!(controller.show_category_bar());
    // Switch to Search mode
    controller.handle_search("smile");
    assert!(!controller.show_category_bar());
    // Clear search (should return to Browse mode)
    controller.handle_search("");
    assert!(controller.show_category_bar());
}
mod tests {
        #[test]
        fn test_listener_callback_receives_correct_state() {
            use std::cell::RefCell;
            use std::rc::Rc;
            let mut controller = make_controller();
            let called = Rc::new(RefCell::new(false));
            let called_clone = Rc::clone(&called);
            controller.add_listener(move |mode, emojis| {
                *called_clone.borrow_mut() = true;
                // Should be in Search mode and filtered_emojis should match query
                assert_eq!(mode, PickerMode::Search);
                assert!(emojis.iter().all(|e| e.name_en.contains("joy") || e.keywords_en.iter().any(|k| k.contains("joy"))));
            });
            controller.handle_search("joy");
            assert!(*called.borrow(), "Listener callback was not called");
        }
    use super::*;
    use crate::emoji::emoji_data::EMOJIS;

    fn make_controller() -> EmojiPickerController {
        let all_emojis = EMOJIS.to_vec();
        EmojiPickerController::new(all_emojis)
    }

    #[test]
    fn test_initial_mode_is_browse() {
        let controller = make_controller();
        assert_eq!(controller.mode, PickerMode::Browse);
        assert!(controller.search_query.is_empty());
        assert!(controller.filtered_emojis.is_empty());
    }

    #[test]
    fn test_search_switches_to_search_mode_and_filters() {
        let mut controller = make_controller();
        controller.handle_search("smile");
        assert_eq!(controller.mode, PickerMode::Search);
        assert_eq!(controller.search_query, "smile");
        assert!(controller
            .filtered_emojis
            .iter()
            .all(|e| e.name_en.contains("smile")
                || e.keywords_en.iter().any(|k| k.contains("smile"))
                || e.name_nl.contains("smile")
                || e.keywords_nl.iter().any(|k| k.contains("smile"))
                || e.ch.contains("smile")));
    }

    #[test]
    fn test_search_empty_switches_to_browse_mode() {
        let mut controller = make_controller();
        controller.handle_search("smile");
        assert_eq!(controller.mode, PickerMode::Search);
        controller.handle_search("");
        assert_eq!(controller.mode, PickerMode::Browse);
        assert!(controller.filtered_emojis.is_empty());
    }

    #[test]
    fn test_search_switching_multiple_times() {
        let mut controller = make_controller();
        // Start in browse
        assert_eq!(controller.mode, PickerMode::Browse);
        // Search
        controller.handle_search("joy");
        assert_eq!(controller.mode, PickerMode::Search);
        assert!(!controller.filtered_emojis.is_empty());
        // Clear search
        controller.handle_search("");
        assert_eq!(controller.mode, PickerMode::Browse);
        assert!(controller.filtered_emojis.is_empty());
        // Search again
        controller.handle_search("face");
        assert_eq!(controller.mode, PickerMode::Search);
        assert!(!controller.filtered_emojis.is_empty());
    }
}

/*
Event flow:
- User types in SearchBar: on_search callback updates controller state, triggers search, updates EmojiGrid, hides/shows CategoryBar.
- User clicks a category: on_category_selected callback updates EmojiGrid to show that category.
- User selects an emoji: on_emoji_selected callback handles the action (e.g., copy to clipboard).
*/
