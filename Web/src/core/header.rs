extern crate wacm;

use wacm::Component;
use super::display_language_selector;
use super::nav_bar;
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
    let nav_bar = nav_bar::get_component();

    return Component {
        css: format!(
            "header {{
            padding: 0 0 0 100px;
            }}
            {display_language_selector}
            {nav_bar}",
            display_language_selector = display_language_selector.css,
            nav_bar = nav_bar.css
        ),
        html: format!(
            "<header>
            {display_language_selector}
            <h1>{banner}</h1>
            {nav_bar}
            </header>",
            banner = TITLE.get_display_text(),
            display_language_selector = display_language_selector.html,
            nav_bar = nav_bar.html
        ),
    }
}
