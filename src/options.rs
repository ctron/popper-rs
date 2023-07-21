//! Options
use crate::{console, modifier::*, prelude::*};
use wasm_bindgen::prelude::*;

/// Options for creating the popper instance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Options {
    /// Desired placement
    pub placement: Placement,
    /// Positioning strategy
    pub strategy: Strategy,
    /// Modifiers
    pub modifiers: Vec<Modifier>,
}

impl TryFrom<Options> for JsValue {
    type Error = JsValue;

    fn try_from(value: Options) -> Result<Self, Self::Error> {
        create_opts(&value)
    }
}

impl TryFrom<&Options> for JsValue {
    type Error = JsValue;

    fn try_from(value: &Options) -> Result<Self, Self::Error> {
        create_opts(value)
    }
}

#[doc(hidden)]
pub fn create_opts(opts: &Options) -> Result<JsValue, JsValue> {
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

    console::debug!("Options after conversion:", &result);

    Ok(result.into())
}
