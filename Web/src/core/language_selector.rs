extern crate wacm;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use wacm::Component;
use crate::repos::display_language;

static DISPLAY_LIST_ITEMS: AtomicBool = AtomicBool::new(false);

static AVAILABLE_LANGUAGES: [Language; 3] = [
    Language {
        id: 1,
        display_text: "English",
    },
    Language {
        id: 2,
        display_text: "Farsi",
    },
    Language {
        id: 3,
        display_text: "Svenska",
    },
];

struct Language {
    id: i32,
    display_text: &'static str,
}

pub fn get_language_selector() -> Component {
    let list_item_elements = if DISPLAY_LIST_ITEMS.load(Ordering::Relaxed) {
        format!(
            "<ul>
            <li onclick='window.nemidoonam.set_language(1)'>{en}</li>
            <li onclick='window.nemidoonam.set_language(2)'>{fa}</li>
            <li onclick='window.nemidoonam.set_language(3)'>{sw}</li>
            </ul>",
            en = AVAILABLE_LANGUAGES[0].display_text,
            fa = AVAILABLE_LANGUAGES[1].display_text,
            sw = AVAILABLE_LANGUAGES[2].display_text,
        )
    } else {
        "".to_string()
    };

    let display_language_id = display_language::get();

    return Component {
        css: format!(
            ".language-selector {{
            display: flex;
            justify-content: flex-end;
            }}
            .language-selector ul {{
            list-style:none;
            position: absolute;
            z-index: 10;
            padding-inline-start: 0px;
            }}
            .language-selector div {{
            width: 80px;
            }}"
        ),
        html: format!(
            "<div class='language-selector' onclick='window.nemidoonam.handle_language_selector_click()'>
            <div>
            {display_language}
            {list_item_elements}
            </div>
            </div>",
            display_language = AVAILABLE_LANGUAGES.iter().find(|&l| l.id == display_language_id).unwrap().display_text,
            list_item_elements = list_item_elements
        ),
    }
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
