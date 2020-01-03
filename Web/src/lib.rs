extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wacm;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wacm::Component;

mod core;
mod repos;
mod localization;
mod pages;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn start_app() -> JsValue {
    let header = core::header::get_header();
    let page = pages::page::get_current_page();

    let app_model = Component {
        css: format!(
            "{header}
            {page}",
            header = header.css,
            page = page.css
        ),
        html: format!(
            "{header}
            {page}",
            header = header.html,
            page = page.html
        )
    };

    return JsValue::from_serde(&app_model).unwrap();
}
