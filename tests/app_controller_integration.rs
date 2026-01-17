//! Integration tests for EmojiPickerController and GTK-dependent UI logic.

use emoji_picker::ui::app_controller::{EmojiPickerController, PickerMode};
use emoji_picker::emoji::emoji_data::EMOJIS;
use std::rc::Rc;
use std::cell::RefCell;

fn make_controller() -> Rc<RefCell<EmojiPickerController>> {
    let all_emojis = EMOJIS.to_vec();
    Rc::new(RefCell::new(EmojiPickerController::new(all_emojis)))
}

#[test]
fn test_initial_mode_is_browse() {
    let controller = make_controller();
    assert_eq!(controller.borrow().mode, PickerMode::Browse);
}

#[test]
fn test_search_changes_mode_and_filters() {
    let controller = make_controller();
    controller.borrow_mut().handle_search("smile");
    assert_eq!(controller.borrow().mode, PickerMode::Search);
    assert!(controller.borrow().filtered_emojis.iter().any(|e| e.name_en.contains("smile")));
}

#[test]
fn test_empty_search_restores_browse_mode() {
    let controller = make_controller();
    controller.borrow_mut().handle_search("");
    assert_eq!(controller.borrow().mode, PickerMode::Browse);
}
