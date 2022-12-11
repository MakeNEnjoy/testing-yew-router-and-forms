use yew::{prelude::*};
use serde::{Serialize, Deserialize};
use yew_router::prelude::*;
use gloo_console::log;
use web_sys::{HtmlInputElement};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/songs/:id")]
    Song { id: String },
    #[at("/songs/search")]
    SongSearch,
}

fn home_page() -> Html {
    html! {
        <>
            <h1>{ "Home" }</h1>
            <SongByID />
            <SearchSongByName />
        </>
    }
}

fn song_page(id: String) -> Html {
    html! {
        <>
            <h1> { "Get songs by id" } </h1>
            <SongByID />
            <h2>{ "Song" }</h2>
            <p>{ id }</p>
            // Get song data from API.
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => home_page(),
        Route::Song { id } => song_page(id),
        Route::SongSearch => song_by_names_page(),
    }
}

#[function_component(SongByID)]
fn song_by_id() -> Html {
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

#[function_component(SearchSongByName)]
fn search_song_by_name() -> Html {
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

fn song_by_names_page() -> Html {
    html! {
        <>
            <h1>{ "Song Search" }</h1>
            <SearchSongByName />
            <SongsByName />
        </>
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SongNameQuery {
    term: String,
}

#[function_component(SongsByName)]
fn songs_by_name() -> Html {
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


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}