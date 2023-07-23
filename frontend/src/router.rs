use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Copy)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/rust")]
    Rust,
    #[at("/me")]
    Me,
    #[at("/daddy")]
    Daddy,
    #[at("/test")]
    Test,
    #[not_found]
    #[at("/404")]
    NotFound,
}
