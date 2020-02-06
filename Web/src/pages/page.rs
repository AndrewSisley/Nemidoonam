extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wacm::Component;
use crate::repos::{ current_page, pages };

pub trait Page<'a> {
    fn get_title(&self) -> &'a str;
    fn get_page(&self) -> Component;
}

const PAGE_CLASS_NAME: &'static str = "page-body";

pub fn get_current_page() -> Component {
    let current_page_id = current_page::get();
    let page_content = pages::get()[current_page_id as usize].get_page();

    Component {
        css: format!(
            ".{page_class_name} {{
            margin: 0 100px 0 100px;
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
    current_page::set(id);
}
