use wasm_bindgen::prelude::*;

pub mod types;

#[cfg_attr(debug_assertions, wasm_bindgen(module = "/js/debug/popperjs.js"))]
#[cfg_attr(
    not(debug_assertions),
    wasm_bindgen(module = "/js/release/popperjs.js")
)]
extern "C" {

    #[wasm_bindgen(js_name = "createPopper")]
    pub fn create_popper(
        reference: web_sys::Node,
        popper: web_sys::Node,
        opts: &JsValue,
    ) -> Instance;

    pub type Instance;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Instance);

    #[wasm_bindgen(method)]
    pub async fn update(this: &Instance);

    #[wasm_bindgen(method, js_name = "forceUpdate")]
    pub fn force_update(this: &Instance);

}
