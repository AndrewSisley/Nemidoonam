use wasm_bindgen::prelude::*;
use wacm::Component;
use super::display_language_selector;
use super::nav_bar;
use super::target_language_selector;
use crate::localization::{ label::Label, label_definition::LabelDefinition };
use crate::repos::{ display_language, target_language };

static TITLE: LabelDefinition = LabelDefinition {
    english: Label {
        display_text: "Welcome",
    },
    farsi: Label {
        display_text: "خوش امدی",
    },
    swedish: Label {
        display_text: "Välkommen",
    },
};

pub fn get_header() -> Component {
    let display_language_selector = display_language_selector::get_language_selector();
    let target_language_selector = target_language_selector::get_component();
    let nav_bar = nav_bar::get_component();

    return Component {
        css: format!(
            "header {{
            padding: 0 100px 0 100px;
            }}
            .menu-right {{
            display: flex;
            justify-content: flex-end;
            direction: ltr;
            }}
            .language-select-spacer {{
            margin: 0 20px 0 0;
            padding: 0px 10px 4px 10px;
            }}
            .language-select-spacer:hover {{
            background-color: lightgray;
            }}
            {display_language_selector}
            {target_language_selector}
            {nav_bar}",
            display_language_selector = display_language_selector.css,
            target_language_selector = target_language_selector.css,
            nav_bar = nav_bar.css
        ),
        html: format!(
            "<header>
            <div class='menu-right'>
            {display_language_selector}
            <span class='language-select-spacer' onclick='window.nemidoonam.reverse_languages()'>=></span>
            {target_language_selector}
            </div>
            <h1>{banner}</h1>
            {nav_bar}
            </header>",
            banner = TITLE.get_display_text(),
            display_language_selector = display_language_selector.html,
            target_language_selector = target_language_selector.html,
            nav_bar = nav_bar.html
        ),
    }
}

#[wasm_bindgen]
pub fn reverse_languages() {
    let display_language_id = display_language::get();
    let target_language_id = target_language::get();

    display_language::set(target_language_id);
    target_language::set(display_language_id);
}
