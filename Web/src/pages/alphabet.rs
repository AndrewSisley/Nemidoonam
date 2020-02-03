use wacm::Component;
use crate::localization::{ label::Label, label_definition::LabelDefinition };
use crate::repos::{ learning_items };

const TABLE_CLASS: &'static str = "alphapet-table";

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
    let mut learning_items = learning_items::get();

    learning_items.sort_unstable_by(
        |a, b| a.get_target_text().partial_cmp(b.get_target_text()).unwrap()
    );

    let mut learning_item_string_builder = format!("<table class={table_class}><tbody>", table_class = TABLE_CLASS);

    for learning_item in &learning_items {
        let item_element = format!(
            "<tr>
            <td>{target_text}</td>
            <td>=></td>
            <td>{display_text}</td>
            </tr>",
            display_text = learning_item.get_display_text(),
            target_text = learning_item.get_target_text(),
        );
        learning_item_string_builder = format!("{}{}", learning_item_string_builder, item_element);
    }

    let learning_item_elements = learning_item_string_builder + "</tbody></table>";

    return Component {
        css: format!(
            ".{table_class} td {{
            min-width: 80px;
            text-align: center;
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
