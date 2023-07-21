use popper_rs::modifier::Modifier;
use popper_rs::options::Options;
use popper_rs::prelude::*;
use popper_rs::sys::{create_popper, Instance};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::Element;

pub fn main() -> Result<(), JsValue> {
    let button = gloo_utils::document().get_element_by_id("button");
    let tooltip = gloo_utils::document().get_element_by_id("tooltip");

    if let Some((button, tooltip)) = button.zip(tooltip) {
        let instance = create_popper(
            &button,
            &tooltip,
            &Options {
                modifiers: vec![Modifier::Offset(Offset {
                    skidding: 0,
                    distance: 8,
                })],
                ..Default::default()
            }
            .try_into()
            .expect("Failed to convert"),
        );

        let instance = Rc::new(instance);

        fn show(tooltip: &Element, popper: &Instance) {
            let _ = tooltip.set_attribute("data-show", "");

            let popper = popper.clone();
            let future = async move {
                popper.update().await;
            };

            wasm_bindgen_futures::spawn_local(future);
        }

        fn hide(tooltip: &Element, _popper: &Instance) {
            let _ = tooltip.remove_attribute("data-show");
        }

        const SHOW_EVENTS: &[&str] = &["mouseenter", "focus"];
        const HIDE_EVENTS: &[&str] = &["mouseleave", "blur"];

        for event in SHOW_EVENTS {
            let instance = instance.clone();
            let tooltip = tooltip.clone();
            gloo_events::EventListener::new(&button, *event, move |_| show(&tooltip, &instance))
                .forget();
        }
        for event in HIDE_EVENTS {
            let instance = instance.clone();
            let tooltip = tooltip.clone();
            gloo_events::EventListener::new(&button, *event, move |_| hide(&tooltip, &instance))
                .forget();
        }

        {
            instance.force_update();
            let state = instance.state();
            web_sys::console::log_2(&"State".into(), &state);

            web_sys::console::log_2(&"Attributes".into(), &state.attributes());
            web_sys::console::log_2(&"Attributes/Popper".into(), &state.attributes().popper());

            web_sys::console::log_2(&"Styles".into(), &state.styles());
            web_sys::console::log_2(&"Styles/Arrow".into(), &state.styles().popper());
            web_sys::console::log_2(&"Styles/Popper".into(), &state.styles().arrow());
        }
    }

    Ok(())
}
