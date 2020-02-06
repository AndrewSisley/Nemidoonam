use wacm::Component;
use crate::localization::{ label::Label, label_definition::LabelDefinition };
use crate::pages::page::{ Page };

pub static TITLE: LabelDefinition = LabelDefinition {
    english: Label {
        display_text: "Home",
    },
    farsi: Label {
        display_text: "خانه",
    },
    swedish: Label {
        display_text: "Hem",
    },
};

pub struct Home;

impl<'a> Page<'a> for Home {
    fn get_title(&self) -> &'a str {
        TITLE.get_display_text()
    }

    fn get_page(&self) -> Component {
        Component {
            css: format!(
                ""
            ),
            html: format!(
                "<h2>{title}</h2>",
                title = TITLE.get_display_text(),
            ),
        }
    }
}
