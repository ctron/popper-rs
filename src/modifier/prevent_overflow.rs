use serde_json::{Value, json};

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
