extern crate wacm;

use wacm::Component;
use super::display_language_selector;
use super::nav_bar;
use super::target_language_selector;
use crate::localization::{ label::Label, label_definition::LabelDefinition };

static TITLE: LabelDefinition = LabelDefinition {
    id: 1,
    english: Label {
        display_text: "Welcome",
    },
    farsi: Label {
        display_text: "Khosh amadid",
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
            padding: 0 0 0 100px;
            }}
            .menu-right {{
            display: flex;
            justify-content: flex-end;
            }}
            .language-select-spacer {{
            padding: 0 20px 0 0;
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
            <span class='language-select-spacer'>=></span>
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
