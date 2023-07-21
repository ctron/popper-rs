mod offset;
mod prevent_overflow;

pub use offset::*;
pub use prevent_overflow::*;

use crate::sys::ModifierArguments;
use gloo_utils::format::JsValueSerdeExt;
use serde_json::json;
use std::borrow::Cow;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct ModifierFn(#[allow(clippy::type_complexity)] pub Rc<Closure<dyn Fn(ModifierArguments)>>);

impl PartialEq for ModifierFn {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    Custom {
        name: Cow<'static, str>,
        phase: Option<Cow<'static, str>>,
        enabled: Option<bool>,
        r#fn: Option<ModifierFn>,
    },
    Offset(Offset),
    PreventOverflow(PreventOverflow),
}

impl Modifier {
    pub fn to_value(&self) -> Result<JsValue, JsValue> {
        let options = match self {
            Self::Custom {
                name,
                phase,
                enabled,
                r#fn,
            } => {
                let m1 = js_sys::Object::new();
                js_sys::Reflect::set(&m1, &"name".into(), &JsValue::from_str(name))?;
                if let Some(phase) = phase {
                    js_sys::Reflect::set(&m1, &"phase".into(), &JsValue::from_str(phase))?;
                }
                if let Some(enabled) = enabled {
                    js_sys::Reflect::set(&m1, &"enabled".into(), &JsValue::from_bool(*enabled))?;
                }
                if let Some(r#fn) = r#fn {
                    js_sys::Reflect::set(&m1, &"fn".into(), (*r#fn.0).as_ref())?;
                }

                return Ok(m1.into());
            }
            Self::Offset(options) => {
                json!({
                    "name": "offset",
                    "options": options.to_json(),
                })
            }
            Self::PreventOverflow(options) => {
                json!({
                    "name": "preventOverflow",
                    "options": options.to_json(),
                })
            }
        };

        JsValue::from_serde(&options).map_err(|err| JsValue::from_str(&err.to_string()))
    }
}
