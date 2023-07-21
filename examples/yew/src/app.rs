use crate::example::{basic, component, ondemand, portal};
use popper_rs::prelude::Placement;
use yew::prelude::*;

#[function_component(Application)]
pub fn application() -> Html {
    html!(
        <>
            <h1>{"popper-rs example for Yew"}</h1>

            <section>
                <h2>{"Basic"}</h2>

                <basic::Example id="example1" target="Hover me">{ "Content" }</basic::Example>
                <basic::Example id="example2" target="Hover me too" placement={Placement::Bottom}>{ "Different content" }</basic::Example>
            </section>

            <section>
                <h2>{"On demand"}</h2>

                <ondemand::Example id="example1" target="Hover me">{ "Content" }</ondemand::Example>
                <ondemand::Example id="example2" target="Hover me too" placement={Placement::Bottom}>{ "Different content" }</ondemand::Example>
            </section>

            <section>
                <h2>{"Portal"}</h2>

                <portal::Example id="example1" target="Hover me">{ "Content" }</portal::Example>
                <portal::Example id="example2" target="Hover me too" placement={Placement::Bottom}>{ "Different content" }</portal::Example>
            </section>

            <section>
                <h2>{"Component"}</h2>

                <component::Example id="example1" target="Click me">{ "Content" }</component::Example>
                <component::Example id="example2" target="Click me too" placement={Placement::Bottom}>{ "Different content" }</component::Example>
            </section>
        </>
    )
}
