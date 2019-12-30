extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wacm;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wacm::Component;

mod localization;

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
    let r = Component {
      html: format!("<div>placeholder text</div>")
    };

    return JsValue::from_serde(&r).unwrap();
}
