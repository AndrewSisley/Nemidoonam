use wacm::Component;

#[derive(Default)]
pub struct DropdownItem<'a> {
    pub id: i32,
    pub display_text: &'a str,
}

pub struct DropdownItemCollection<'a> {
    pub handle_on_click_method_name: &'a str,
    pub items: &'a [DropdownItem<'a>],
}

pub fn get_component(display_text: &str, handle_on_click: &str, display_list_items: bool, dropdown_items: &DropdownItemCollection) -> Component {
    let list_item_elements = if display_list_items {
        let mut list_item_string_builder = "<ul>".to_string();

        for item in dropdown_items.items {
            let item_element = format!(
                "<li onclick='{handle_on_click_method_name}({id})'>{display_text}</li>",
                handle_on_click_method_name = dropdown_items.handle_on_click_method_name,
                id = item.id,
                display_text = item.display_text
            );
            list_item_string_builder = format!("{}{}", list_item_string_builder, item_element);
        }

        list_item_string_builder + "</ul>"
    } else {
        "".to_string()
    };

    return Component {
        css: format!(
            ".dropdown ul {{
            list-style:none;
            position: absolute;
            z-index: 10;
            padding-inline-start: 0px;
            margin-block-start: 4px;
            width: 80px;
            }}
            .dropdown li {{
            padding: 0px 0 4px 10px;
            margin: 0 0 0 -10px;
            background-color: white;
            }}
            .dropdown li:hover {{
            background-color: lightgray;
            }}
            .dropdown div {{
            width: 80px;
            padding: 0px 0 4px 10px;
            }}
            .dropdown div:hover {{
            background-color: lightgray;
            }}"
        ),
        html: format!(
            "<div class='dropdown' onclick='{handle_on_click}'>
            <div>
            {display_text}
            {list_item_elements}
            </div>
            </div>",
            handle_on_click = handle_on_click,
            display_text = display_text,
            list_item_elements = list_item_elements
        ),
    }
}
