use crate::types::Placement;
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
        reference: &web_sys::Node,
        popper: &web_sys::Node,
        opts: &JsValue,
    ) -> Instance;

    #[derive(Clone, Debug)]
    pub type Instance;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Instance);

    #[wasm_bindgen(method)]
    pub async fn update(this: &Instance);

    #[wasm_bindgen(method, js_name = "forceUpdate")]
    pub fn force_update(this: &Instance);

    #[wasm_bindgen(method, getter)]
    pub fn state(this: &Instance) -> State;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type State;

    #[wasm_bindgen(method, getter)]
    pub fn attributes(this: &State) -> Attributes;

    #[wasm_bindgen(method, getter)]
    pub fn styles(this: &State) -> Styles;

    #[wasm_bindgen(method, getter)]
    pub fn placement(this: &State) -> Placement;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Styles;

    #[wasm_bindgen(method, getter)]
    pub fn arrow(this: &Styles) -> JsValue;
    #[wasm_bindgen(method, getter)]
    pub fn popper(this: &Styles) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Attributes;

    #[wasm_bindgen(method, getter)]
    pub fn popper(this: &Attributes) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type ModifierArguments;

    #[wasm_bindgen(method, getter)]
    pub fn instance(this: &ModifierArguments) -> Instance;
}
