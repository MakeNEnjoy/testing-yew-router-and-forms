use gloo_console::log;
use web_sys::{HtmlInputElement};
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct SongNameQuery {
    term: String,
}

#[function_component(SongsByName)]
pub fn songs_by_name() -> Html {
    let location = use_location().unwrap();
    match location.query::<SongNameQuery>() {
        Ok(query) => {
            html! {
                <>
                    <h2>{ format!("List of songs with name {}", query.term) }</h2>
                    <p>{ "..." }</p>
                </>
            }
        }
        Err(_) => html! {
            <>
                <h1>{ "Song Search" }</h1>
                <p>{ "No query" }</p>
            </>
        }
    }

}

#[function_component(SearchSongByName)]
pub fn search_song_by_name() -> Html {
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
                let song = Route::SongSearch;
                let query = SongNameQuery { term: input.value() };
                let result = navigator.replace_with_query(&song, &query);
                match result {
                    Ok(_) => log!("song search by query was successful."),
                    Err(e) => log!("unsuccefully serialised query: {}", e.to_string()),
                }

            }
        })
    };
    html!{
        <form onsubmit = {onsubmit} >
            <input ref = {input_node_ref} type="text" required=true/>
            <input type="submit" value = "Get Song by name"/>
        </form>
    }
}

pub fn song_by_names_page() -> Html {
    html! {
        <>
            <h1>{ "Song Search" }</h1>
            <SearchSongByName />
            <SongsByName />
        </>
    }
}