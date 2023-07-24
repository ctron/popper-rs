use super::inner::InnerPopper;
use crate::prelude::*;
use yew::prelude::*;

/// A component showing the popper content in a portal.
///
/// ## Usage
///
/// When visible, the component will show its children in a portal attached to the body of the
/// document. The user has the responsibility to apply the styles and attributes. In order to apply
/// them, it is necessary to capture the state and apply it to the child components. It is also
/// necessary to provide a [`NodeRef`], which references the popper content, so that the calculation
/// works correctly.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use popper_rs::prelude::*;
/// use popper_rs::yew::component::PortalPopper;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     /// The reference and content
///     let button_ref = use_node_ref();
///     let tooltip_ref = use_node_ref();
///
///     // For activating the popper
///     let active = use_state_eq(|| false);
///     let onclick = {
///         let active = active.clone();
///         use_callback(move |_, active| active.set(!**active), active)
///     };
///
///     // For passing the state
///     let state = use_state_eq(State::default);
///     let onstatechange = use_callback(|new_state, state| state.set(new_state), state.clone());
///
///     html!(
///         <div>
///             <button
///                 ref={button_ref.clone()}
///                 class="button"
///                 {onclick}
///             > { "Click me" } </button>
///
///             <PortalPopper
///                 target={button_ref}
///                 content={tooltip_ref.clone()}
///                 visible={*active}
///                 {onstatechange}
///             >
///                 <div
///                     ref={tooltip_ref}
///                     data-show="true"
///                     style={&state.styles.popper}
///                 >
///                     { "Tooltip content" }
///                     <div style={&state.styles.arrow}></div>
///                 </div>
///             </PortalPopper>
///         </div>
///     )
/// }
/// ```
///
/// For a complete example, see the `yew` example `component`.
#[function_component(PortalPopper)]
pub fn portal_popper(props: &PopperProperties) -> Html {
    if props.visible {
        html!(<InnerPopper
            base={props.clone()}
            strategy={Strategy::Fixed}
            portal=true
        />)
    } else {
        Html::default()
    }
}
