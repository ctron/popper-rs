use crate::sys;
use std::rc::Rc;
use yew::prelude::*;

pub struct UsePopper {
    pub instance: Rc<sys::Instance>,
}

#[hook]
pub fn use_popper() -> UsePopper {}
