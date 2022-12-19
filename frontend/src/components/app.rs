use yew::prelude::*;
use yew_router::prelude::*;

use super::guessing_page::GuessingPage;
use super::login::Login;
use super::signup::Signup;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    GuessingPage,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: &Route) -> Html {
    match routes {
        Route::Signup => html! {
            <Signup />
        },
        Route::Login => html! {
            <Login />
        },
        Route::GuessingPage => html! {
            <GuessingPage />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
