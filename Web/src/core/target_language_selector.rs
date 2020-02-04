extern crate wacm;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use wacm::Component;
use crate::repos::{ target_language, available_languages };
use crate::components::dropdown;

static DISPLAY_LIST_ITEMS: AtomicBool = AtomicBool::new(false);

pub fn get_component() -> Component {
    let available_languages = available_languages::get();
    let target_language_id = target_language::get();

    let mut items: [dropdown::DropdownItem; available_languages::LANGUAGE_COUNT] = Default::default();

    let mut i: usize = 0;
    for available_language in &available_languages {
        items[i] = dropdown::DropdownItem {
            id: available_language.id,
            display_text: available_language.get_display_text(),
        };
        i = i + 1;
    }

    let dropdown_items = dropdown::DropdownItemCollection {
        handle_on_click_method_name: "window.nemidoonam.set_target_language",
        items: &items,
    };

    return dropdown::get_component(
        available_languages.iter().find(|&l| l.id == target_language_id).unwrap().get_display_text(),
        "window.nemidoonam.handle_target_language_selector_click()",
        DISPLAY_LIST_ITEMS.load(Ordering::Relaxed),
        &dropdown_items
    );
}

#[wasm_bindgen]
pub fn handle_target_language_selector_click() {
    let was_true = DISPLAY_LIST_ITEMS.compare_and_swap(true, false, Ordering::Relaxed);
    if !was_true {
        DISPLAY_LIST_ITEMS.compare_and_swap(false, true, Ordering::Relaxed);
    }
}

#[wasm_bindgen]
pub fn set_target_language(id: i32) {
    target_language::set(id);
}
