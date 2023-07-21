use popper_rs::modifier::{Modifier, Offset};
use popper_rs::prelude::{Options, Placement};
use popper_rs::state::ApplyAttributes;
use popper_rs::yew::use_popper;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ExampleProperties {
    pub id: AttrValue,
    pub target: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub placement: Placement,
}

#[function_component(Example)]
pub fn example(props: &ExampleProperties) -> Html {
    let ids = use_memo(
        |id| (format!("button-{id}"), format!("tooltip-{id}")),
        props.id.clone(),
    );

    let button_ref = use_node_ref();
    let tooltip_ref = use_node_ref();

    let options = use_memo(
        |placement| Options {
            placement: *placement,
            modifiers: vec![Modifier::Offset(Offset {
                skidding: 0,
                distance: 8,
            })],
            ..Default::default()
        },
        props.placement,
    );

    let popper = use_popper(button_ref.clone(), tooltip_ref.clone(), options).unwrap();

    let active = use_state_eq(|| false);

    let onshow = {
        let active = active.clone();
        use_callback(
            move |(), (active, popper)| {
                let popper = popper.clone();
                spawn_local(async move {
                    popper.update().await;
                });
                active.set(true);
            },
            (active, popper.instance),
        )
    };

    let onhide = {
        let active = active.clone();
        use_callback(move |(), active| active.set(false), active)
    };

    let data_show = (*active).then(AttrValue::default);

    use_effect_with_deps(
        |(tooltip_ref, attributes)| {
            tooltip_ref.apply_attributes(attributes);
        },
        (tooltip_ref.clone(), popper.state.attributes.popper.clone()),
    );

    html!(
        <>
            <div>
                <button
                    ref={button_ref}
                    id={ids.0.to_string()}
                    class="button"
                    aria-describedby={ids.1.to_string()}
                    onmouseover={onshow.reform(|_|())} onfocus={onshow.reform(|_|())}
                    onmouseout={onhide.reform(|_|())} onblur={onhide.reform(|_|())}
                > { &props.target } </button>

                <div
                    data-show={&data_show}
                    ref={tooltip_ref}
                    id={ids.1.to_string()}
                    role="tooltip"
                    class="tooltip"
                    style={&popper.state.styles.popper}
                >
                    { for props.children.iter() }
                    <div class="arrow" data-popper-arrow="true" style={&popper.state.styles.arrow}></div>
                </div>
            </div>
            <div>
                { format!("{:#?}", &*popper.state) }
            </div>
        </>
    )
}
