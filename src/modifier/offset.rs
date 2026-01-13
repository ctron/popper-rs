use serde_json::{Value, json};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Offset {
    pub skidding: i32,
    pub distance: i32,
}

impl Offset {
    pub fn to_json(&self) -> Value {
        json!({
            "offset": [self.skidding, self.distance],
        })
    }
}
