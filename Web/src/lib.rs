use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wacm::Component;

mod core;
mod repos;
mod localization;
mod pages;
mod components;
mod learning;

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
    let shared_css = core::styles::get_shared_css();

    let app_model = Component {
        css: format!(
            "{shared_css}
            {header}
            {page}",
            shared_css = shared_css,
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
