use wacm::Component;
use crate::localization::{ label::Label, label_definition::LabelDefinition };

pub static TITLE: LabelDefinition = LabelDefinition {
    id: 2,
    english: Label {
        display_text: "Home",
    },
    farsi: Label {
        display_text: "Khane",
    },
    swedish: Label {
        display_text: "Hem",
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
