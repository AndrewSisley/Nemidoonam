extern crate wacm;

use wacm::Component;
use super::language_selector;
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
    let language_selector = language_selector::get_language_selector();

    return Component {
        css: format!(
            "header {{
            padding: 0 0 0 100px;
            }}
            {language_selector}",
            language_selector = language_selector.css
        ),
        html: format!(
            "<header>
            {language_selector}
            <h1>{banner}</h1>
            </header>",
            banner = TITLE.get_display_text(),
            language_selector = language_selector.html
        ),
    }
}
