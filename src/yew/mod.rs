//! Yew integration

pub mod component;

use crate::{console, prelude::*, sys};
use popper_rs_sys::ModifierArguments;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Clone)]
pub struct UsePopper {
    pub instance: UsePopperInstance,
    pub state: UseStateHandle<State>,
}

#[derive(Clone)]
pub struct UsePopperInstance(pub Rc<RefCell<Option<Instance>>>);

impl PartialEq for UsePopperInstance {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl UsePopperInstance {
    pub fn force_update(&self) {
        if let Some(instance) = &*self.0.borrow() {
            instance.force_update();
        }
    }

    pub async fn update(&self) {
        if let Some(instance) = &*self.0.borrow() {
            instance.update().await;
        }
    }
}

#[hook]
pub fn use_popper(
    reference: NodeRef,
    popper: NodeRef,
    options: Rc<Options>,
) -> Result<UsePopper, JsValue> {
    let state = use_state_eq(State::default);

    // on state callback, we must keep this until we no longer need the popper instance
    let onstatechange = {
        use_memo(
            |()| {
                let state = state.clone();
                Closure::wrap(Box::new(move |args: ModifierArguments| {
                    let s = args.instance().state();
                    console::debug!("updateState:", &s);
                    state.set(s.into());
                }) as Box<dyn Fn(ModifierArguments)>)
            },
            (),
        )
    };

    let options: Rc<Result<JsValue, JsValue>> = {
        use_memo(
            |(opts, onstatechange)| {
                let mut opts: Options = (**opts).clone();

                // disable applying styles, we do this ourselves
                opts.modifiers.push(Modifier::Custom {
                    name: "applyStyles".into(),
                    phase: None,
                    enabled: Some(false),
                    r#fn: None,
                });
                // capture state, forward to callbacks
                opts.modifiers.push(Modifier::Custom {
                    name: "updateState".into(),
                    phase: Some("write".into()),
                    enabled: Some(true),
                    r#fn: Some(onstatechange.clone()),
                });

                opts.try_into()
            },
            (options, ModifierFn(onstatechange.clone())),
        )
    };

    let options = match options.as_ref() {
        Ok(options) => options.clone(),
        Err(err) => return Err(err.clone()),
    };

    /*
    use_effect_with_deps(
        |(r1, r2)| {
            console::debug!("R1", r1.get());
            console::debug!("R2", r2.get());
        },
        (reference.clone(), popper.clone()),
    );*/

    let instance = use_mut_ref(|| Option::<Instance>::None);

    {
        let instance = instance.clone();
        use_effect_with_deps(
            move |(reference, popper, options)| {
                let reference = reference.get();
                let popper = popper.get();

                let result = if let (Some(reference), Some(popper)) = (reference, popper) {
                    console::debug!("Creating popper instance");
                    let instance = sys::create_popper(&reference, &popper, options);
                    instance.force_update();
                    Some(instance)
                } else {
                    None
                };

                *instance.borrow_mut() = result.clone();

                move || {
                    if let Some(result) = result {
                        console::debug!("Destroying popper instance");
                        result.destroy();
                    }
                }
            },
            (reference, popper, options),
        );
    }

    Ok(UsePopper {
        instance: UsePopperInstance(instance),
        state,
    })
}
