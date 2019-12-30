extern crate wacm;

use wacm::Component;
use crate::localization::{ label::Label, label_definition::LabelDefinition };

static TITLE: LabelDefinition = LabelDefinition {
    id: 1,
    english: Label {
        display_text: "Welcome",
    },
    farsi: Label {
        display_text: "Khiosh amadi",
    },
    swedish: Label {
        display_text: "Välkommen",
    },
};

pub fn get_header() -> Component {
    return Component {
        css: format!(
            "header {{
            padding: 0 0 0 100px;
            }}"
        ),
        html: format!(
            "<header>
            <h1>{banner}</h1>
            </header>",
            banner = TITLE.get_display_text()
        ),
    }
}
