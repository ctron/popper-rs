mod console;

pub mod modifier;
pub mod options;
pub mod state;

use crate::options::Options;
pub use popper_rs_sys as sys;
use std::convert::TryInto;
use wasm_bindgen::JsValue;

pub mod prelude {
    pub use popper_rs_sys::types::*;
    pub use popper_rs_sys::Instance;

    pub use crate::modifier::*;
    pub use crate::options::Options;
}

#[cfg(feature = "yew")]
pub mod yew;

pub fn create(
    reference: &web_sys::Node,
    popper: &web_sys::Node,
    opts: &Options,
) -> Result<prelude::Instance, JsValue> {
    Ok(sys::create_popper(reference, popper, &opts.try_into()?))
}
