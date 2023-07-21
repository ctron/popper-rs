use crate::example::{basic, ondemand};
use popper_rs::prelude::Placement;
use yew::prelude::*;

#[function_component(Application)]
pub fn application() -> Html {
    html!(
        <>
            <h1>{"popper-rs example for Yew"}</h1>

            <h2>{"Basic"}</h2>

            <basic::Example id="example1" target="Hover me">{ "Content" }</basic::Example>
            <basic::Example id="example2" target="Hover me too" placement={Placement::Bottom}>{ "Different content" }</basic::Example>

            <h2>{"On demand"}</h2>

            <ondemand::Example id="example1" target="Hover me">{ "Content" }</ondemand::Example>
            <ondemand::Example id="example2" target="Hover me too" placement={Placement::Bottom}>{ "Different content" }</ondemand::Example>
        </>
    )
}
