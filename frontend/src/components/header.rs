use yew::{function_component, html, Callback, Html, Properties};
use yew_router::prelude::{use_navigator, use_route};

use crate::router::Route;

#[function_component]
pub fn Navbar() -> Html {
    html! {
    <div class="container">
      <header class="d-flex justify-content-center py-3">
        <ul class="nav nav-pills">
          <Navlink name="Home" route={Route::Home}/>
          <Navlink name="Rust" route={Route::Rust}/>
          <Navlink name="Me" route={Route::Me}/>
          <Navlink name="Daddy" route={Route::Daddy}/>
          <Navlink name="test" route={Route::Test}/>
        </ul>
      </header>
    </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavlinkProps {
    pub name: String,
    pub route: Route,
}

#[function_component]
pub fn Navlink(props: &NavlinkProps) -> Html {
    let current_route = use_route::<Route>().unwrap_or_default();
    let navigator = use_navigator().unwrap();

    let class = if current_route == props.route {
        "nav-link active"
    } else {
        "nav-link"
    };

    let route = props.route.clone();
    let onclick = Callback::from(move |_| navigator.push(&route));

    html! {
        <li class="nav-item"><button {onclick} class={class}>{&props.name}</button></li>
    }
}
