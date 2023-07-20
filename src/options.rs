use crate::{console, modifier::*, prelude::*};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub placement: Placement,
    pub strategy: Strategy,
    pub modifiers: Vec<Modifier>,
}

pub fn create_opts(opts: Options) -> Result<JsValue, JsValue> {
    let mods = js_sys::Array::new();
    for m in &opts.modifiers {
        mods.push(&m.to_value()?);
    }

    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &JsValue::from("modifiers"), &mods)?;
    js_sys::Reflect::set(
        &result,
        &JsValue::from_str("strategy"),
        &opts.strategy.into(),
    )?;
    js_sys::Reflect::set(
        &result,
        &JsValue::from_str("placement"),
        &opts.placement.into(),
    )?;

    console::debug!("options:", &result);

    Ok(result.into())
}
