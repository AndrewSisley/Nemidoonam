use wacm::Component;
use crate::localization::{ label::Label, label_definition::LabelDefinition };

pub static TITLE: LabelDefinition = LabelDefinition {
    english: Label {
        display_text: "Alphabet",
    },
    farsi: Label {
        display_text: "Alaf be",
    },
    swedish: Label {
        display_text: "Alfabet",
    },
};

pub fn get_page() -> Component {
    return Component {
        css: format!(
            ""
        ),
        html: format!(
            "<h2>{title}</h2>",
            title = TITLE.get_display_text(),
        ),
    }
}
