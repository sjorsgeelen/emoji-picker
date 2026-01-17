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
}


impl EmojiPickerController {
    pub fn new(all_emojis: Vec<Emoji>) -> Self {
        Self {
            mode: PickerMode::Browse,
            search_query: String::new(),
            all_emojis,
            filtered_emojis: Vec::new(),
        }
    }

    /// Handle a search query update. Switches mode and updates filtered results.
    pub fn handle_search(&mut self, query: &str) {
        self.search_query = query.to_string();
        if query.is_empty() {
            self.mode = PickerMode::Browse;
            self.filtered_emojis.clear();
        } else {
            self.mode = PickerMode::Search;
            let filtered: Vec<_> = self.all_emojis.iter()
                .filter(|e| e.name_en.contains(query) || e.keywords_en.iter().any(|k| k.contains(query)))
                .cloned().collect();
            self.filtered_emojis = filtered;
        }
    }

    pub fn handle_category_selected(&mut self, _category_idx: usize) {
        // In real UI, would update grid
    }

    pub fn handle_emoji_selected(&mut self, _emoji_idx: usize) {
        // In real UI, would handle emoji selection
    }
}

#[cfg(test)]
mod tests {
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
        assert!(controller.filtered_emojis.iter().all(|e| e.name_en.contains("smile") || e.keywords_en.iter().any(|k| k.contains("smile"))));
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

