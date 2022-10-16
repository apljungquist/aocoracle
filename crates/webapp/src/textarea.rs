use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn textarea(props: &Props) -> Html {
    let Props { value, on_change } = props.clone();

    let onfocus = Callback::from(move |event: FocusEvent| {
        let target: HtmlTextAreaElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
        web_sys::console::log_1(&target.value().into());
        target.select();
    });

    let oninput = Callback::from(move |event: InputEvent| {
        let target: HtmlTextAreaElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
        on_change.emit(target.value());
    });

    html! {
        <textarea type="text" {value} {onfocus} {oninput}/>
    }
}
