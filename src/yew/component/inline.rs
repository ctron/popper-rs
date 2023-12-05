use super::{inner::InnerPopper, PopperProperties};
use crate::prelude::*;
use yew::prelude::*;

/// A component showing the popper content inline.
#[function_component(InlinePopper)]
pub fn inline_popper(props: &PopperProperties) -> Html {
    if props.visible {
        html!(<InnerPopper
            base={props.clone()}
            strategy={Strategy::Absolute}
            portal=false
            portal_target={None}
        />)
    } else {
        Html::default()
    }
}
