use yew::prelude::*;
use yew_router::prelude::*;

use super::guessing_page::GuessingPage;
use super::login::Login;
use super::signup::Signup;
use crate::entities::interfaces::User;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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

#[derive(Debug, PartialEq, Clone)]
struct AppState {
    opt_user: Option<User>,
}

#[derive(Properties, PartialEq, Debug)]
pub struct AppProps {
    pub opt_user: Option<User>,
}

fn user_logged_in(appstate: &AppState) -> bool {
    match appstate.opt_user {
        Some(_)=> true,
        None => false,
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| AppState{opt_user: None});
    let cb_user_login: Callback<User> = {
        let state = state.clone();
        Callback::from(move |user: User| {
            let greeting = format!("Hey, {} !", user.t_email);
            log::info!("{}", greeting);
            state.set(AppState { opt_user: Some(user) });
        })
    };
    let switch = {
        let state = state.clone();
        Switch::render(move |routes: &Route|{
            match routes {
                Route::Signup => html! {
                    <Signup />
                },
                Route::Login => {
                    let cb_user_login = cb_user_login.clone();
                    html! {
                        <Login {cb_user_login} />
                    }
                },
                Route::GuessingPage => {
                    let opt_user = state.opt_user.clone();
                    html! {
                        <GuessingPage {opt_user} />
                    }
                },
                Route::NotFound => html! { <h1>{ "404" }</h1> },
            }
        })
    };
    html! {
        <>
            <BrowserRouter>
                <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">
                        <h1 class="navbar-item is-size-3">{ "Wikifind" }</h1>
                        // <button class={classes!("navbar-burger", "burger", active_class)}
                        //     aria-label="menu" aria-expanded="false"
                        //     onclick={link.callback(|_| Msg::ToggleNavbar)}
                        // >
                        //     <span aria-hidden="true"></span>
                        //     <span aria-hidden="true"></span>
                        //     <span aria-hidden="true"></span>
                        // </button>
                        <div class="navbar-start">
                        {
                            if user_logged_in(&state) {
                                html!{}
                            } else {
                                html!{
                                    <Link<Route> classes={classes!("navbar-item")} to={Route::Login}>
                                        { "Login" }
                                    </Link<Route>>
                                }
                            }
                        }
                        </div>
                    </div>
                </nav>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}
