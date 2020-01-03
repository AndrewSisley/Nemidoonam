extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wacm::Component;
use super::{ home, alphabet };
use crate::repos;

pub fn get_current_page() -> Component {
    let current_page_id = repos::current_page::get();
    return get_page(current_page_id);
}

fn get_page(page_id: i32) -> Component {
    match page_id {
        1 => home::get_page(),
        2 => alphabet::get_page(),
        _ => home::get_page(),
    }
}

#[wasm_bindgen]
pub fn set_page(id: i32) {
    repos::current_page::set(id);
}