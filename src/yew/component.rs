use crate::console;
use crate::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct PortalPopperProperties {
    /// The content to show when the popper is visible
    #[prop_or_default]
    pub children: Children,

    /// Flag if the content should be visible
    pub visible: bool,

    /// The target to align the content to
    pub target: NodeRef,

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
pub fn portal_popper(props: &PortalPopperProperties) -> Html {
    if props.visible {
        html!(<InnerPopper ..props.clone() />)
    } else {
        Html::default()
    }
}

#[function_component(InnerPopper)]
fn inner_popper(props: &PortalPopperProperties) -> Html {
    let popper_ref = props.content.clone();
    let reference_ref = props.target.clone();

    let options = use_memo(
        |(placement, modifiers)| Options {
            placement: *placement,
            strategy: Strategy::Fixed,
            modifiers: modifiers.clone(),
            ..Default::default()
        },
        (props.placement, props.modifiers.clone()),
    );

    let popper = use_popper(reference_ref.clone(), popper_ref.clone(), options).unwrap();

    {
        let popper = popper.instance.clone();
        use_effect(|| {
            spawn_local(async move {
                popper.update().await;
            });
        });
    }

    use_effect_with_deps(
        |(callback, state)| {
            console::debug!("Forwarding state change", format!("{:?}", state));
            callback.emit(state.clone());
        },
        (props.onstatechange.clone(), (*popper.state).clone()),
    );

    create_portal(
        html!(
            { for props.children.iter() }
        ),
        gloo_utils::body().into(),
    )
}
