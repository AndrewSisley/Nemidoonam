use crate::learning::{ learning_item::LearningItem };
use crate::localization::{ label::Label, label_definition::LabelDefinition };
use crate::repos::{ target_language, available_languages::ids as languages };

pub fn get() -> Vec<LearningItem> {
    let target_language_id = target_language::get();

    return get_all()
        .iter()
        .filter(
            |&learning_item| learning_item.target_languages
                .iter()
                .any(
                    |&item_target| item_target == target_language_id
                )
        )
        .map(
            |learning_item| LearningItem {
                target_languages: learning_item.target_languages.to_vec(),
                label_definition: LabelDefinition {
                    english: Label {
                        display_text: learning_item.label_definition.english.display_text,
                    },
                    farsi: Label {
                        display_text: learning_item.label_definition.farsi.display_text,
                    },
                    swedish: Label {
                        display_text: learning_item.label_definition.swedish.display_text,
                    },
                },
            }
        )
        .collect()
}

fn get_all() -> [LearningItem; 43] {
    [
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "a",
                },
                farsi: Label {
                    display_text: "ا",
                },
                swedish: Label {
                    display_text: "a",
                },
            }
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "b",
                },
                farsi: Label {
                    display_text: "ب",
                },
                swedish: Label {
                    display_text: "b",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "p",
                },
                farsi: Label {
                    display_text: "پ",
                },
                swedish: Label {
                    display_text: "p",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "t",
                },
                farsi: Label {
                    display_text: "ت",
                },
                swedish: Label {
                    display_text: "t",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "s",
                },
                farsi: Label {
                    display_text: "ث",
                },
                swedish: Label {
                    display_text: "s",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "j",
                },
                farsi: Label {
                    display_text: "ج",
                },
                swedish: Label {
                    display_text: "j",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "ch",
                },
                farsi: Label {
                    display_text: "چ",
                },
                swedish: Label {
                    display_text: "tj",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "h",
                },
                farsi: Label {
                    display_text: "ح",
                },
                swedish: Label {
                    display_text: "h",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "kh",
                },
                farsi: Label {
                    display_text: "خ",
                },
                swedish: Label {
                    display_text: "kh",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "d",
                },
                farsi: Label {
                    display_text: "د",
                },
                swedish: Label {
                    display_text: "d",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "z",
                },
                farsi: Label {
                    display_text: "ذ",
                },
                swedish: Label {
                    display_text: "z",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "r",
                },
                farsi: Label {
                    display_text: "ر",
                },
                swedish: Label {
                    display_text: "r",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "z",
                },
                farsi: Label {
                    display_text: "ز",
                },
                swedish: Label {
                    display_text: "z",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "zh",
                },
                farsi: Label {
                    display_text: "ژ",
                },
                swedish: Label {
                    display_text: "zh",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "s",
                },
                farsi: Label {
                    display_text: "س",
                },
                swedish: Label {
                    display_text: "s",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "sh",
                },
                farsi: Label {
                    display_text: "ش",
                },
                swedish: Label {
                    display_text: "sh",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "s",
                },
                farsi: Label {
                    display_text: "ص",
                },
                swedish: Label {
                    display_text: "s",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "z",
                },
                farsi: Label {
                    display_text: "ض",
                },
                swedish: Label {
                    display_text: "z",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "t",
                },
                farsi: Label {
                    display_text: "ط",
                },
                swedish: Label {
                    display_text: "t",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "z",
                },
                farsi: Label {
                    display_text: "ظ",
                },
                swedish: Label {
                    display_text: "z",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "'a",
                },
                farsi: Label {
                    display_text: "ع",
                },
                swedish: Label {
                    display_text: "'a",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "gh",
                },
                farsi: Label {
                    display_text: "غ",
                },
                swedish: Label {
                    display_text: "gh",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "f",
                },
                farsi: Label {
                    display_text: "ف",
                },
                swedish: Label {
                    display_text: "f",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "q",
                },
                farsi: Label {
                    display_text: "ق",
                },
                swedish: Label {
                    display_text: "q",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "k",
                },
                farsi: Label {
                    display_text: "ک",
                },
                swedish: Label {
                    display_text: "k",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "g",
                },
                farsi: Label {
                    display_text: "گ",
                },
                swedish: Label {
                    display_text: "g",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "l",
                },
                farsi: Label {
                    display_text: "ل",
                },
                swedish: Label {
                    display_text: "l",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "m",
                },
                farsi: Label {
                    display_text: "م",
                },
                swedish: Label {
                    display_text: "m",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "n",
                },
                farsi: Label {
                    display_text: "ن",
                },
                swedish: Label {
                    display_text: "n",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::FARSI,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "v/w/u/o",
                },
                farsi: Label {
                    display_text: "و",
                },
                swedish: Label {
                    display_text: "v/w/u/o",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "h",
                },
                farsi: Label {
                    display_text: "ه",
                },
                swedish: Label {
                    display_text: "h",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::FARSI,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "y",
                },
                farsi: Label {
                    display_text: "ی",
                },
                swedish: Label {
                    display_text: "y",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "c",
                },
                farsi: Label {
                    display_text: "س/ک",
                },
                swedish: Label {
                    display_text: "c",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "e",
                },
                farsi: Label {
                    display_text: "ی",
                },
                swedish: Label {
                    display_text: "e",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "i",
                },
                farsi: Label {
                    display_text: "ی",
                },
                swedish: Label {
                    display_text: "i",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "o",
                },
                farsi: Label {
                    display_text: "و",
                },
                swedish: Label {
                    display_text: "o",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "u",
                },
                farsi: Label {
                    display_text: "و",
                },
                swedish: Label {
                    display_text: "u",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "v",
                },
                farsi: Label {
                    display_text: "و",
                },
                swedish: Label {
                    display_text: "v",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "w",
                },
                farsi: Label {
                    display_text: "و",
                },
                swedish: Label {
                    display_text: "w",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::ENGLISH,
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "x",
                },
                farsi: Label {
                    display_text: "ایکس ",
                },
                swedish: Label {
                    display_text: "x",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "o",
                },
                farsi: Label {
                    display_text: "و",
                },
                swedish: Label {
                    display_text: "å",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "ae",
                },
                farsi: Label {
                    display_text: "عه",
                },
                swedish: Label {
                    display_text: "ä",
                },
            },
        },
        LearningItem {
            target_languages: vec![
                languages::SWEDISH,
            ],
            label_definition: LabelDefinition {
                english: Label {
                    display_text: "uhh",
                },
                farsi: Label {
                    display_text: "وه",
                },
                swedish: Label {
                    display_text: "ö",
                },
            },
        },
    ]
}
