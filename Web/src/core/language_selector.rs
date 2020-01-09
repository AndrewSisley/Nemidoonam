extern crate wacm;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use wacm::Component;
use crate::repos::{ display_language, available_languages };
use crate::components::dropdown;

static DISPLAY_LIST_ITEMS: AtomicBool = AtomicBool::new(false);

pub fn get_language_selector() -> Component {
    let available_languages = available_languages::get();
    let display_language_id = display_language::get();

    let dropdown_items = dropdown::DropdownItemCollection {
        handle_on_click_method_name: "window.nemidoonam.set_language",
        items: &[
            dropdown::DropdownItem {
                id: available_languages[0].id,
                display_text: available_languages[0].get_display_text(),
            },
            dropdown::DropdownItem {
                id: available_languages[1].id,
                display_text: available_languages[1].get_display_text(),
            },
            dropdown::DropdownItem {
                id: available_languages[2].id,
                display_text: available_languages[2].get_display_text(),
            },
        ],
    };

    return dropdown::get_component(
        available_languages.iter().find(|&l| l.id == display_language_id).unwrap().get_display_text(),
        "window.nemidoonam.handle_language_selector_click()",
        DISPLAY_LIST_ITEMS.load(Ordering::Relaxed),
        &dropdown_items
    );
}

#[wasm_bindgen]
pub fn handle_language_selector_click() {
    let was_true = DISPLAY_LIST_ITEMS.compare_and_swap(true, false, Ordering::Relaxed);
    if !was_true {
        DISPLAY_LIST_ITEMS.compare_and_swap(false, true, Ordering::Relaxed);
    }
}

#[wasm_bindgen]
pub fn set_language(id: i32) {
    display_language::set(id);
}
