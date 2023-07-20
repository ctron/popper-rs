use serde_json::{json, Value};

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize)]
pub struct PreventOverflow {
    pub padding: i32,
}

impl PreventOverflow {
    pub fn to_json(&self) -> Value {
        json!({
            "padding": self.padding,
        })
    }
}
