mod offset;
mod prevent_overflow;

pub use offset::*;
pub use prevent_overflow::*;

use crate::sys::Instance;
use gloo_utils::format::JsValueSerdeExt;
use serde_json::json;
use std::borrow::Cow;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

pub type ModifierFn = Rc<Closure<dyn Fn(&Instance)>>;

#[derive(Clone, Debug)]
pub enum Modifier {
    Custom {
        name: Cow<'static, str>,
        phase: Cow<'static, str>,
        r#fn: ModifierFn,
    },
    Offset(Offset),
    PreventOverflow(PreventOverflow),
}

impl Modifier {
    pub fn to_value(&self) -> Result<JsValue, JsValue> {
        let options = match self {
            Self::Custom { name, phase, r#fn } => {
                let m1 = js_sys::Object::new();
                js_sys::Reflect::set(&m1, &JsValue::from("name"), &JsValue::from_str(name))?;
                js_sys::Reflect::set(&m1, &JsValue::from("phase"), &JsValue::from_str(phase))?;
                js_sys::Reflect::set(&m1, &JsValue::from("fn"), (**r#fn).as_ref())?;
                return Ok(m1.into());
            }
            Self::Offset(options) => {
                json!({
                    "name": "offset",
                    "phase": "main",
                    "options": options.to_json(),
                })
            }
            Self::PreventOverflow(options) => {
                json!({
                    "name": "preventOverflow",
                    "phase": "main",
                    "options": options.to_json(),
                })
            }
        };

        JsValue::from_serde(&options).map_err(|err| JsValue::from_str(&err.to_string()))
    }
}
