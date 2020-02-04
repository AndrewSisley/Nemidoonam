extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wacm::Component;
use super::{ home, alphabet };
use crate::repos;

const PAGE_CLASS_NAME: &'static str = "page-body";

pub fn get_current_page() -> Component {
    let current_page_id = repos::current_page::get();
    return get_page(current_page_id);
}

fn get_page(page_id: i32) -> Component {
    let page_content = match page_id {
        1 => home::get_page(),
        2 => alphabet::get_page(),
        _ => home::get_page(),
    };

    Component {
        css: format!(
            ".{page_class_name} {{
            margin: 0 0 0 100px;
            }}
            {page_content}",
            page_class_name = PAGE_CLASS_NAME,
            page_content = page_content.css
        ),
        html: format!(
            "<section class='{page_class_name}'>
            {page_content}
            </section>",
            page_class_name = PAGE_CLASS_NAME,
            page_content = page_content.html
        )
    }
}

#[wasm_bindgen]
pub fn set_page(id: i32) {
    repos::current_page::set(id);
}