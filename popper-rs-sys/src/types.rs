use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    Bottom = "bottom",
    Top = "top",
    Left = "left",
    Right = "right",
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Right
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Placement {
    Auto = "auto",
    AutoStart = "auto-start",
    AutoEnd = "auto-end",

    Left = "left",
    LeftStart = "left-start",
    LeftEnd = "left-end",

    Top = "top",
    TopStart = "top-start",
    TopEnd = "top-end",

    Right = "right",
    RightStart = "right-start",
    RightEnd = "right-end",

    Bottom = "bottom",
    BottomStart = "bottom-start",
    BottomEnd = "bottom-end",
}

impl Default for Placement {
    fn default() -> Self {
        Self::Auto
    }
}

/// Positioning strategy for the popper content.
///
/// This relates to the positioning of the HTML element, using the CSS `position` style.
///
/// When using "portals", you most likely want [`Strategy::Fixed`].
#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Strategy {
    Absolute = "absolute",
    Fixed = "fixed",
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Absolute
    }
}
