use super::inner::InnerPopper;
use crate::prelude::*;
use std::ops::Deref;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PortalToPopperProperties {
    /// The node to append the content to
    pub append_to: Option<web_sys::Element>,
    /// The popper properties
    pub popper: PopperProperties,
    /// The content to show
    pub children: Children,
}

impl Deref for PortalToPopperProperties {
    type Target = PopperProperties;

    fn deref(&self) -> &Self::Target {
        &self.popper
    }
}

/// A component showing the popper content in a portal appended to a specific element.
///
/// ## Usage
///
/// When visible, the component will show its children in a portal attached to the provided element.
/// The user has the responsibility to apply the styles and attributes. In order to apply
/// them, it is necessary to capture the state and apply it to the child components. It is also
/// necessary to provide a [`NodeRef`], which references the popper content, so that the calculation
/// works correctly.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use popper_rs::prelude::*;
/// use popper_rs::yew::component::PortalToPopper;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     /// The references
///     let button_ref = use_node_ref();
///     let tooltip_ref = use_node_ref();
///     let content_ref = use_node_ref();
///
///     // For activating the popper
///     let active = use_state_eq(|| false);
///     let onclick = use_callback(active.clone(), move |_, active| active.set(!**active));
///
///     // For passing the state
///     let state = use_state_eq(State::default);
///     let onstatechange = use_callback(state.clone(), |new_state, state| state.set(new_state));
///
///     html!(
///         <div>
///             <button
///                 ref={button_ref.clone()}
///                 class="button"
///                 {onclick}
///             > { "Click me" } </button>
///
///             <div ref={content_ref.clone()}></div>
///
///             <PortalToPopper
///                 popper={yew::props!(PopperProperties {
///                     target: button_ref,
///                     content: tooltip_ref.clone(),
///                     visible: *active,
///                     onstatechange,
///                 })}
///                 append_to={content_ref.cast::<web_sys::Element>()}
///             >
///                 <div
///                     ref={tooltip_ref}
///                     data-show="true"
///                     style={&state.styles.popper}
///                 >
///                     { "Tooltip content" }
///                     <div style={&state.styles.arrow}></div>
///                 </div>
///             </PortalToPopper>
///         </div>
///     )
/// }
/// ```
#[function_component(PortalToPopper)]
pub fn portal_to_popper(props: &PortalToPopperProperties) -> Html {
    if props.visible {
        html!(<InnerPopper
            base={PopperProperties {
                children: props.children.clone(),
                ..props.popper.clone()
            }}
            strategy={Strategy::Absolute}
            portal=true
            portal_target={props.append_to.clone()}
        />)
    } else {
        Html::default()
    }
}
