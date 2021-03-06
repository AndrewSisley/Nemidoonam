use wasm_bindgen::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use wacm::Component;
use crate::repos::{ display_language, available_languages };
use crate::components::dropdown;

static DISPLAY_LIST_ITEMS: AtomicBool = AtomicBool::new(false);

pub fn get_language_selector() -> Component {
    let available_languages = available_languages::get();
    let display_language_id = display_language::get();

    let mut items: [dropdown::DropdownItem; available_languages::LANGUAGE_COUNT] = Default::default();

    for index in 0..available_languages.len() {
        let available_language = &available_languages[index];
        items[index] = dropdown::DropdownItem {
            id: available_language.id,
            display_text: available_language.get_display_text(),
        };
    }

    let dropdown_items = dropdown::DropdownItemCollection {
        handle_on_click_method_name: "window.nemidoonam.set_display_language",
        items: &items,
    };

    return dropdown::get_component(
        available_languages.iter().find(|&l| l.id == display_language_id).unwrap().get_display_text(),
        "window.nemidoonam.handle_display_language_selector_click()",
        DISPLAY_LIST_ITEMS.load(Ordering::Relaxed),
        &dropdown_items
    );
}

#[wasm_bindgen]
pub fn handle_display_language_selector_click() {
    let was_true = DISPLAY_LIST_ITEMS.compare_and_swap(true, false, Ordering::Relaxed);
    if !was_true {
        DISPLAY_LIST_ITEMS.compare_and_swap(false, true, Ordering::Relaxed);
    }
}

#[wasm_bindgen]
pub fn set_display_language(id: i32) {
    display_language::set(id);
}
