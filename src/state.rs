use crate::{console, prelude::Orientation};
use popper_rs_sys::Instance;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub orientation: Orientation,
    pub styles: String,
}

pub fn from_instance(popper: &Instance) -> Result<State, JsValue> {
    from_popper(popper)
}

pub(crate) fn from_popper(popper: impl AsRef<JsValue>) -> Result<State, JsValue> {
    let state = js_sys::Reflect::get(popper.as_ref(), &JsValue::from("state"))?;
    let attributes = js_sys::Reflect::get(&state, &JsValue::from_str("attributes"))?;
    let popper = js_sys::Reflect::get(&attributes, &JsValue::from("popper"))?;
    let placement = js_sys::Reflect::get(&popper, &JsValue::from("data-popper-placement"))?;

    let orientation = Orientation::from_js_value(&placement).unwrap_or(Orientation::Left);

    console::debug!(
        "Orientation - original:",
        placement.as_string(),
        "outcome:",
        orientation
    );

    let styles = js_sys::Reflect::get(&state, &JsValue::from_str("styles"))?;
    let popper = js_sys::Reflect::get(&styles, &JsValue::from("popper"))?;

    let popper = js_sys::Object::from(popper);

    let mut styles: String = js_sys::Object::entries(&popper)
        .to_vec()
        .iter()
        .map(js_sys::Array::from)
        .map(|field| {
            let key = js_sys::Array::get(&field, 0);
            let value = js_sys::Array::get(&field, 1);
            (
                key.as_string().unwrap_or_default(),
                value.as_string().unwrap_or_default(),
            )
        })
        .map(|(key, value)| format!("{}: {};", key, value))
        .collect::<Vec<String>>()
        .join(" ");

    styles.push_str(" z-index: 1000;");

    console::debug!("Computed Style:", &styles);

    Ok(State {
        orientation,
        styles,
    })
}
