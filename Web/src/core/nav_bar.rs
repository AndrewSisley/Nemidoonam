extern crate wacm;

use wacm::Component;
use crate::repos::{ pages };

pub fn get_component() -> Component {
    let pages = pages::get();
    let mut nav_item_string_builder = String::new();

    for page_index in 0..pages.len() {
        let page_element = format!(
            "<a onclick='window.nemidoonam.set_page({page_id})'>{page_title}</a>",
            page_id = page_index,
            page_title = pages[page_index].get_title(),
        );
        nav_item_string_builder = format!("{}{}", nav_item_string_builder, page_element);
    }

    nav_item_string_builder.to_string();

    return Component {
        css: ".nav-bar-top {
        display: inline-block;
        background-color: lightgray;
        }
        .nav-bar-top a:hover {
        background-color: grey;
        }
        .nav-bar-top a {
        display: inline-block;
        font-size: 1.3em;
        padding: 5px 20px 5px 20px;
        }".to_string(),
        html: format!(
            "<nav class='nav-bar-top'>
            {nav_items}
            </nav>",
            nav_items = nav_item_string_builder,
        ),
    }
}
