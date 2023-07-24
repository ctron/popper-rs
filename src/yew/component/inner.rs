use super::PopperProperties;
use crate::{console, prelude::*};
use std::ops::Deref;
use yew::{platform::spawn_local, prelude::*};

#[derive(PartialEq, Properties)]
pub(crate) struct InnerPopperProperties {
    pub base: PopperProperties,
    pub strategy: Strategy,
    pub portal: bool,
}

impl Deref for InnerPopperProperties {
    type Target = PopperProperties;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[function_component(InnerPopper)]
pub(crate) fn inner_popper(props: &InnerPopperProperties) -> Html {
    let popper_ref = props.content.clone();
    let reference_ref = props.target.clone();

    let options = use_memo(
        |(placement, modifiers)| Options {
            placement: *placement,
            strategy: props.strategy,
            modifiers: modifiers.clone(),
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

    if props.portal {
        create_portal(
            html!(
                { for props.children.iter() }
            ),
            gloo_utils::body().into(),
        )
    } else {
        html!(
            { for props.children.iter() }
        )
    }
}
