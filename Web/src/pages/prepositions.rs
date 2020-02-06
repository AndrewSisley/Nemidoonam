use wacm::Component;
use crate::localization::{ label::Label, label_definition::LabelDefinition };
use crate::pages::page::{ Page };
use crate::repos::{ learning_items, text_direction, modules::Module };

const TABLE_CLASS: &'static str = "prepositions-table";

pub static TITLE: LabelDefinition = LabelDefinition {
    english: Label {
        display_text: "Prepositions",
    },
    farsi: Label {
        display_text: "حروف اضافه",
    },
    swedish: Label {
        display_text: "Prepositioner",
    },
};

pub struct Prepositions;

impl<'a> Page<'a> for Prepositions {
    fn get_title(&self) -> &'a str {
        TITLE.get_display_text()
    }

    fn get_page(&self) -> Component {
        let display_text_direction = text_direction::get_display();
        let learning_items = learning_items::get(Module::Prepositions);

        let mut learning_item_string_builder = format!("<table class='{table_class}'><tbody>", table_class = TABLE_CLASS);

        for learning_item in &learning_items {
            let item_element = if display_text_direction == text_direction::LEFT_TO_RIGHT {
                format!(
                    "<tr>
                    <td class='target-text'>{target_text}</td>
                    <td>=></td>
                    <td>{display_text}</td>
                    </tr>",
                    display_text = learning_item.get_display_text(),
                    target_text = learning_item.get_target_text(),
                )
            } else {
                format!(
                    "<tr>
                    <td>{display_text}</td>
                    <td>=></td>
                    <td class='target-text'>{target_text}</td>
                    </tr>",
                    display_text = learning_item.get_display_text(),
                    target_text = learning_item.get_target_text(),
                )
            };
            learning_item_string_builder = format!("{}{}", learning_item_string_builder, item_element);
        }

        let learning_item_elements = learning_item_string_builder + "</tbody></table>";

        return Component {
            css: format!(
                ".{table_class} td {{
                padding: 0 10px 0 10px;
                font-size: 2em;
                }}",
                table_class = TABLE_CLASS
            ),
            html: format!(
                "<h2>{title}</h2>
                {learning_item_elements}",
                title = TITLE.get_display_text(),
                learning_item_elements = learning_item_elements
            ),
        }
    }
}
