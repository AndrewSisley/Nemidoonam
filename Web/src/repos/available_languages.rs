use crate::localization::{
    label::Label,
    label_definition::LabelDefinition,
    language:: Language
};

pub const LANGUAGE_COUNT: usize = 3;

pub mod ids {
    pub const ENGLISH: i32 = 1;
    pub const FARSI: i32 = 2;
    pub const SWEDISH: i32 = 3;
}

pub fn get() -> [Language; LANGUAGE_COUNT] {
    return [
        Language {
            id: ids::ENGLISH,
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "English",
                },
                farsi: Label {
                    display_text: "انگلیسی",
                },
                swedish: Label {
                    display_text: "Engelska",
                },
            },
        },
        Language {
            id: ids::FARSI,
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "Farsi",
                },
                farsi: Label {
                    display_text: "فارسی",
                },
                swedish: Label {
                    display_text: "Persiska",
                },
            },
        },
        Language {
            id: ids::SWEDISH,
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "Swedish",
                },
                farsi: Label {
                    display_text: "سوئدی",
                },
                swedish: Label {
                    display_text: "Svenska   سوئدی",
                },
            },
        },
    ];
}