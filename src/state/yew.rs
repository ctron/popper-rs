use super::*;
use ::yew::html::IntoPropValue;
use ::yew::{AttrValue, NodeRef};

impl IntoPropValue<Option<AttrValue>> for &StylesMap {
    fn into_prop_value(self) -> Option<AttrValue> {
        if self.is_empty() {
            None
        } else {
            Some(AttrValue::from(self.to_string()))
        }
    }
}

impl ApplyAttributes for NodeRef {
    fn apply_attributes(&self, attributes: &AttributesMap) {
        if let Some(element) = self.cast::<Element>() {
            (&element).apply_attributes(attributes);
        }
    }
}
