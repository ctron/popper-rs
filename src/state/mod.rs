#[cfg(feature = "yew")]
mod yew;

pub use self::yew::*;

use crate::sys;
use js_sys::{Array, Object};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StylesMap(pub HashMap<String, String>);

impl Deref for StylesMap {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StylesMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for StylesMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.0 {
            write!(f, "{k}: {v};")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AttributesMap(pub HashMap<String, String>);

pub trait ApplyAttributes {
    fn apply_attributes(&self, attributes: &AttributesMap);
}

impl ApplyAttributes for &Element {
    fn apply_attributes(&self, attributes: &AttributesMap) {
        for (k, v) in &attributes.0 {
            let _ = self.set_attribute(&k, &v);
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub styles: Styles,
    pub attributes: Attributes,
}

impl From<sys::State> for State {
    fn from(value: sys::State) -> Self {
        Self {
            styles: value.styles().into(),
            attributes: value.attributes().into(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Styles {
    pub popper: StylesMap,
    pub arrow: StylesMap,
}

impl From<sys::Styles> for Styles {
    fn from(value: sys::Styles) -> Self {
        Self {
            popper: StylesMap(to_map(value.popper())),
            arrow: StylesMap(to_map(value.arrow())),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes {
    pub popper: AttributesMap,
}

impl From<sys::Attributes> for Attributes {
    fn from(value: sys::Attributes) -> Self {
        Self {
            popper: AttributesMap(to_map(value.popper())),
        }
    }
}

fn to_map(value: JsValue) -> HashMap<String, String> {
    let value: Object = match value.dyn_into() {
        Err(_) => return Default::default(),
        Ok(value) => value,
    };

    let entries = match Object::entries(&value).dyn_into::<Array>() {
        Err(_) => return Default::default(),
        Ok(entries) => entries,
    };

    let mut result = HashMap::new();

    for entry in entries.into_iter() {
        let key = js_sys::Reflect::get_u32(&entry, 0);
        let value = js_sys::Reflect::get_u32(&entry, 1);
        if let (Ok(key), Ok(value)) = (key, value) {
            if let (Some(key), Some(value)) = (key.as_string(), value.as_string()) {
                result.insert(key, value);
            }
        }
    }

    result
}
