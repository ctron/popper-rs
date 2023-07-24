use popper_rs::modifier::{Modifier, Offset};
use popper_rs::prelude::Placement;
use popper_rs::state::{ApplyAttributes, State};
use popper_rs::yew::component::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TooltipProperties {
    pub id: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub state: State,

    pub r#ref: NodeRef,
}

#[function_component(Tooltip)]
pub fn example(props: &TooltipProperties) -> Html {
    let tooltip_ref = props.r#ref.clone();

    use_effect_with_deps(
        |(tooltip_ref, attributes)| tooltip_ref.apply_attributes(attributes),
        (tooltip_ref.clone(), props.state.attributes.popper.clone()),
    );

    html!(
        <div
            ref={tooltip_ref}
            data-show="true"
            id={props.id.clone()}
            role="tooltip"
            class="tooltip"
            style={&props.state.styles.popper}
        >
            { for props.children.iter() }
            <div class="arrow" data-popper-arrow="true" style={&props.state.styles.arrow}></div>
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct ExampleProperties {
    pub id: AttrValue,
    pub target: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub placement: Placement,
}

#[function_component(PortalExample)]
pub fn portal_example(props: &ExampleProperties) -> Html {
    let ids = use_memo(
        |id| (format!("button-{id}"), format!("tooltip-{id}")),
        props.id.clone(),
    );

    let button_ref = use_node_ref();
    let tooltip_ref = use_node_ref();

    let active = use_state_eq(|| false);

    let onclick = {
        let active = active.clone();
        use_callback(move |_, active| active.set(!**active), active)
    };

    let state = use_state_eq(State::default);
    let onstatechange = use_callback(|new_state, state| state.set(new_state), state.clone());

    html!(
        <>
            <div>
                <button
                    ref={button_ref.clone()}
                    id={ids.0.to_string()}
                    class="button"
                    aria-describedby={ids.1.to_string()}
                    {onclick}
                > { &props.target } </button>

                <PortalPopper
                    target={button_ref}
                    content={tooltip_ref.clone()}
                    visible={*active}
                    {onstatechange}
                    placement={props.placement}
                    modifiers={vec![
                        Modifier::Offset(Offset {
                            skidding: 0,
                            distance: 8,
                        })
                    ]}
                >
                    <Tooltip
                        id={ids.1.to_string()}
                        r#ref={tooltip_ref}
                        state={(*state).clone()}
                    >
                        { for props.children.iter() }
                    </Tooltip>
                </PortalPopper>
            </div>
        </>
    )
}

#[function_component(InlineExample)]
pub fn inline_example(props: &ExampleProperties) -> Html {
    let ids = use_memo(
        |id| (format!("button-{id}"), format!("tooltip-{id}")),
        props.id.clone(),
    );

    let button_ref = use_node_ref();
    let tooltip_ref = use_node_ref();

    let active = use_state_eq(|| false);

    let onclick = {
        let active = active.clone();
        use_callback(move |_, active| active.set(!**active), active)
    };

    let state = use_state_eq(State::default);
    let onstatechange = use_callback(|new_state, state| state.set(new_state), state.clone());

    html!(
        <>
            <div>
                <button
                    ref={button_ref.clone()}
                    id={ids.0.to_string()}
                    class="button"
                    aria-describedby={ids.1.to_string()}
                    {onclick}
                > { &props.target } </button>

                <InlinePopper
                    target={button_ref}
                    content={tooltip_ref.clone()}
                    visible={*active}
                    {onstatechange}
                    placement={props.placement}
                    modifiers={vec![
                        Modifier::Offset(Offset {
                            skidding: 0,
                            distance: 8,
                        })
                    ]}
                >
                    <Tooltip
                        id={ids.1.to_string()}
                        r#ref={tooltip_ref}
                        state={(*state).clone()}
                    >
                        { for props.children.iter() }
                    </Tooltip>
                </InlinePopper>
            </div>
        </>
    )
}
