use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/songs/:id")]
    Song { id: String },
    #[at("/search/songs")]
    SongSearch,
    #[at("/SongDeletedSuccessfully")]
    SongDeleted,
    #[at("/CreateSong")]
    CreateSong
}