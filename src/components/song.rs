use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use gloo_console::log;
use reqwasm::http::Request;
use web_sys::{HtmlInputElement};
use serde_json::json;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(PostSong)]
fn post_song() -> Html {
    let id_ref = use_node_ref();
    let name_ref = use_node_ref();
    let popularity_ref = use_node_ref();
    let artists_ref = use_node_ref();
    let id_artists_ref = use_node_ref();
    let duration_ms_ref = use_node_ref();
    let release_date_ref = use_node_ref();

    let onsubmit = {
        let id_ref = id_ref.clone();
        let name_ref = name_ref.clone();
        let popularity_ref = popularity_ref.clone();
        let artists_ref = artists_ref.clone();
        let id_artists_ref = id_artists_ref.clone();
        let duration_ms_ref = duration_ms_ref.clone();
        let release_date_ref = release_date_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            log!("submit");
            let id_element = id_ref.clone().cast::<HtmlInputElement>();
            let name_element = name_ref.clone().cast::<HtmlInputElement>();
            let popularity_element = popularity_ref.clone().cast::<HtmlInputElement>();
            let artists_element = artists_ref.clone().cast::<HtmlInputElement>();
            let id_artists_element = id_artists_ref.clone().cast::<HtmlInputElement>();
            let duration_ms_element = duration_ms_ref.clone().cast::<HtmlInputElement>();
            let release_date_element = release_date_ref.clone().cast::<HtmlInputElement>();
            // log!("input: {:?}", );
            // if let Some(input) = input {
            //     log!("input: {:?}", input.value());
            // }

            let id = match id_element {
                Some(input) => input.value(),
                None => "".to_string()
            };
            let name = match name_element {
                Some(input) => input.value(),
                None => "".to_string()
            };
            let popularity = match popularity_element {
                Some(input) => {
                    let popularity = input.value();
                    match popularity.parse::<i32>() {
                        Ok(popularity) => popularity,
                        Err(_) => 0
                    }
                },
                None => 0
            };
            let artists = match artists_element {
                Some(input) => input.value(),
                None => "".to_string()
            };
            let id_artists = match id_artists_element {
                Some(input) => input.value(),
                None => "".to_string()
            };
            let release_date = match release_date_element {
                Some(input) => input.value(),
                None => "".to_string()
            };
            let duration_ms = match duration_ms_element {
                Some(input) => {
                    let duration_ms = input.value();
                    match duration_ms.parse::<i32>() {
                        Ok(duration_ms) => duration_ms,
                        Err(_) => 0
                    }
                },
                None => 0
            };
            let s = SongStruct {
                id: id,
                name: name,
                popularity: popularity,
                artists: artists,
                id_artists: id_artists,
                duration_ms: 0,
                release_date: release_date
            };

            wasm_bindgen_futures::spawn_local(async move {
                log!("sending: {:?}", serde_json::to_string(&s).unwrap());
                Request::post("/api/songs")
                    .body(serde_json::to_string(&s).unwrap())
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap();
            });


        })
    };

    html! {
        <>
            <form onsubmit = {onsubmit}>
                <label> { "ID" }</label><br />
                <input ref = {id_ref} type="text" required=true/><br />
                <label> { "Name" }</label><br />
                <input ref = {name_ref} type="text" required=true/><br />
                <label> { "Popularity (integer)" }</label><br />
                <input ref = {popularity_ref} type="text" pattern=r"\d+"/><br />
                <label> { "Artists" }</label><br />
                <input ref = {artists_ref} type="text"/><br />
                <label> { "ID artists" }</label><br />
                <input ref = {id_artists_ref} type="text"/><br />
                <label> { "Duration (ms)" }</label><br />
                <input ref = {duration_ms_ref} type="text" pattern=r"\d+"/><br />
                <label> { "Release date" }</label><br />
                <input ref = {release_date_ref} type="text"/><br />
                <input type="submit" value = "Create song"/>
            </form>
        </>
    }
}

pub fn create_song_page() -> Html {
    html! {
        <>
            <h1> { "Create song" } </h1>
            <PostSong />
        </>
    }
}

#[function_component(DeleteSong)]
pub fn delete_song(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::SongDeleted));
    html!{
        <button {onclick}> { "Delete" } </button>
    }
}

#[function_component(UpdateSongInfo)]
pub fn update_song_info(props: &Props) -> Html {
    // let navigator = use_navigator().unwrap();
    // let onclick = Callback::from(move |_| navigator.push(&Route::SongDeleted));
    html!{
        <>
            <h3> { "Update song info"} </h3>
            <form>
                <label> { "Name" }</label><br />
                <input type="text" required=true/><br />
                <label> { "Popularity" }</label><br />
                <input type="text"/><br />
                <label> { "Artists" }</label><br />
                <input type="text"/><br />


                <input type="submit" value = "Update song info"/>
            </form>
        </>
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
struct SongStruct {
    id: String,
    name: String,
    popularity: i32,
    artists: String,
    id_artists: String,
    duration_ms: i32,
    release_date: String,
}

#[function_component(SongInfo)]
pub fn song_info(props: &Props) -> Html {
    let song_id = props.id.clone();
    let song = use_state(|| SongStruct::default() );
    {
        let song = song.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_song: SongStruct = Request::get(&format!("/api/songs/{}", song_id))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                song.set(fetched_song);
            });
            || ()
        }, ());
    }

    html!{
        <>
            <h3> { "Song Info"} </h3>
            <p> { "Name: "} {&song.name} </p>
            <p> { "Popularity: "} {&song.popularity} </p>
            <p> { "Artists: "} {&song.artists} </p>
            <p> { "ID Artists: "} {&song.id_artists} </p>
            <p> { "Duration: "} {&song.duration_ms} </p>
            <p> { "Release Date: "} {&song.release_date} </p>
        </>
    }
}

#[function_component(Song)]
pub fn song(props: &Props) -> Html {


    html!{
        <>
            <h2> { "Displaying information for song id "} {props.id.clone()}  </h2>
            < SongInfo id = {props.id.clone()} />
            < DeleteSong id = {props.id.clone()} />
            < UpdateSongInfo id = {props.id.clone()} />
        </>
    }
}

pub fn song_deleted_page() -> Html {
    html! {
        <>
            <h1> { "Song Deleted Successfully" } </h1>
        </>
    }
}