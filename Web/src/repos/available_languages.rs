use crate::localization::{
    label::Label,
    label_definition::LabelDefinition,
    language:: Language
};

pub fn get() -> [Language; 3] {
    return [
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
}