use yew::{prelude::*};
use yew_router::prelude::*;
use gloo_console::log;
use web_sys::{HtmlInputElement};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/songs/:id")]
    Song { id: String },
}

fn home_page() -> Html {
    html! {
        <>
            <h1>{ "Home" }</h1>
            <SongSearch />
        </>
    }
}

fn song_page(id: String) -> Html {
    html! {
        <>
            <h1> { "Song Search" } </h1>
            <SongSearch />
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
    }
}

#[function_component(SongSearch)]
fn song_search() -> Html {
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
            <input ref = {input_node_ref} type="text"
                name="song_id"
            />
            <input type="submit" value = "Get Song by ID"/>
        </form>
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