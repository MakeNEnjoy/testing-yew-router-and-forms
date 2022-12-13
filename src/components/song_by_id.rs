use yew::{prelude::*};
use yew_router::prelude::*;
use gloo_console::log;
use web_sys::{HtmlInputElement};
use crate::router::Route;
use crate::components::song::Song;

#[function_component(SongByID)]
pub fn song_by_id() -> Html {
    log!("song by id rendered");
    let input_node_ref = use_node_ref();
    let navigator = use_navigator().unwrap();
    let onsubmit = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let input = input_node_ref.clone().cast::<HtmlInputElement>();
            if let Some(input) = input {
                if input.value().is_empty() {
                    log!("input is empty!");
                    return;
                }
                let song = Route::Song { id: input.value() };
                navigator.push(&song);
            }
        })
    };
    html!{
        <form onsubmit = {onsubmit} >
            <input ref = {input_node_ref} type="text" required=true/>
            <input type="submit" value = "Get Song by ID"/>
        </form>
    }
}

pub fn song_by_id_page(id: String) -> Html {
    html! {
        <>
            <h1> { "Get songs by id" } </h1>
            <SongByID />
            <Song id = {id} />
        </>
    }
}