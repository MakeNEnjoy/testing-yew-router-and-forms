use yew::{prelude::*};
// use gloo_console::log;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::song_by_id::{SongByID, song_by_id_page};
use crate::components::song_by_name::{SearchSongByName, song_by_names_page};
use crate::components::song::*;

fn home_page() -> Html {
    html! {
        <>
            <h1>{ "Home" }</h1>
            <SongByID />
            <SearchSongByName />
        </>
    }
}


fn switch(route: Route) -> Html {
    match route {
        Route::Home => home_page(),
        Route::Song { id } => song_by_id_page(id),
        Route::SongSearch => song_by_names_page(),
        Route::SongDeleted => song_deleted_page(),
        Route::CreateSong => create_song_page()
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