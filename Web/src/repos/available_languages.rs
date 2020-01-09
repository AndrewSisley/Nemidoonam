use crate::localization::{
    label::Label,
    label_definition::LabelDefinition,
    language:: Language
};

pub fn get() -> [Language; 3] {
    return [
        Language {
            id: 1,
            label_definition: LabelDefinition {
                id: 3,
                english: Label {
                    display_text: "English",
                },
                farsi: Label {
                    display_text: "Englisi",
                },
                swedish: Label {
                    display_text: "Engelska",
                },
            },
        },
        Language {
            id: 2,
            label_definition: LabelDefinition {
                id: 4,
                english: Label {
                    display_text: "Farsi",
                },
                farsi: Label {
                    display_text: "Farsi",
                },
                swedish: Label {
                    display_text: "Persiska",
                },
            },
        },
        Language {
            id: 3,
            label_definition: LabelDefinition {
                id: 4,
                english: Label {
                    display_text: "Swedish",
                },
                farsi: Label {
                    display_text: "Sweudi",
                },
                swedish: Label {
                    display_text: "Svenska",
                },
            },
        },
    ];
}