use yew::{function_component, html, Html, Properties};

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub active_page: Route,
}

#[derive(Properties, PartialEq)]
pub struct NavlinkProps {

}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    html! {
    <div class="container">
      <header class="d-flex justify-content-center py-3">
        <ul class="nav nav-pills">
          <li class="nav-item">
            <a href="/" class="nav-link active" aria-current={active_page(props.active_page, Route::Home)} >{"Home"}</a>
          </li>
          <li class="nav-item"><a href="/rust" class="nav-link active" aria-current={active_page(props.active_page, Route::Home)} >{"Rust"}</a></li>
          <li class="nav-item"><a href="/me" class="nav-link">{"me"}</a></li>
          <li class="nav-item"><a href="/daddy" class="nav-link">{"daddy"}</a></li>
          <li class="nav-item"><a href="/test" class="nav-link">{"test"}</a></li>
        </ul>
      </header>
    </div>
    }
}

#[function_component]
pub fn Navlink(props: &NavlinkProps) -> Html {
    html! {
    }
}

fn active_page(active_page: Route, page: Route) -> String {
    if active_page == page {
        "\"page\"".to_owned()
    } else {
        "\"false\"".to_owned()
    }
}
