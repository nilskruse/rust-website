mod components;
mod router;

use gloo::console;
use router::Route;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

use crate::components::header::Navbar;

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    value: i64, // This will store the counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <BrowserRouter>
            <Switch<Route> render={route} />
            {add_bootstrap()}
        </BrowserRouter>
        }
    }
}

fn route(routes: Route) -> Html {
    html! {
        <>
        <Navbar/>
        {body(routes)}
        </>
    }
}

fn body(routes: Route) -> Html {
    match routes {
        Route::Home => html! {"home"},
        Route::Rust => html! {"rust"},
        Route::Me => html! {"me"},
        Route::Daddy => html! {"daddy"},
        Route::Test => html! {"test"},
        Route::NotFound => html! {"not found"},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn add_bootstrap() -> Html {
    html! {
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js" integrity="sha384-geWF76RCwLtnZ8qwWowPQNguL3RmwHVBC9FhGdlKrxdiJJigb/j/68SIy3Te4Bkz" crossorigin="anonymous"></script>
    }
}
