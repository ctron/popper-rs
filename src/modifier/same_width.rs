use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SameWidth {}

#[wasm_bindgen(inline_js = r#"
export function popper_rs_same_width() {
    return {
        name: "sameWidth",
        enabled: true,
        phase: "beforeWrite",
        requires: ["computeStyles"],
        fn: ({ state }) => {
          state.styles.popper.width = `${state.rects.reference.width}px`;
        },
        effect: ({ state }) => {
          state.elements.popper.style.width = `${state.elements.reference.offsetWidth}px`;
        }
    };
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = "popper_rs_same_width")]
    fn same_width() -> JsValue;
}

impl SameWidth {
    pub fn new() -> Self {
        Self {}
    }

    pub fn to_js_value(&self) -> JsValue {
        same_width()
    }
}
