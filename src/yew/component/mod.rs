//! Components for Yew

mod inline;
mod inner;
mod portal;

pub use inline::*;
pub use portal::*;

use crate::prelude::*;
use yew::prelude::*;

/// Properties for the popper components.
#[derive(Clone, PartialEq, Properties)]
pub struct PopperProperties {
    /// The content to show when the popper is visible
    #[prop_or_default]
    pub children: Children,

    /// Flag if the content should be visible
    pub visible: bool,

    /// The target to align the content to
    pub target: NodeRef,

    /// The reference to the content, once rendered.
    pub content: NodeRef,

    /// The desired placement
    #[prop_or_default]
    pub placement: Placement,

    /// Additional modifiers
    #[prop_or_default]
    pub modifiers: Vec<Modifier>,

    /// The state notification callback.
    pub onstatechange: Callback<State>,
}
