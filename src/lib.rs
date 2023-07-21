//! A popper.js integration for Rust
//!
//! ## Examples
//!
//! See the examples in the `examples` folder.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod console;

pub mod modifier;
pub mod options;
pub mod state;

#[cfg(feature = "yew")]
pub mod yew;

pub use popper_rs_sys as sys;

use crate::options::Options;
use std::convert::TryInto;
use wasm_bindgen::JsValue;

/// The prelude
pub mod prelude {
    pub use popper_rs_sys::types::*;
    pub use popper_rs_sys::Instance;

    pub use crate::modifier::*;
    pub use crate::options::Options;
    pub use crate::state::*;

    #[cfg(feature = "yew")]
    pub use super::yew::*;
}

/// Create a new popper instance.
pub fn create(
    reference: &web_sys::Node,
    popper: &web_sys::Node,
    opts: &Options,
) -> Result<prelude::Instance, JsValue> {
    Ok(sys::create_popper(reference, popper, &opts.try_into()?))
}
