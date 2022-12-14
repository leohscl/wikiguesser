use std::fmt::Display;
use std::str::FromStr;

use yew::prelude::*;
use yew_router::prelude::*;

use super::launch_page::LaunchPage;
use super::guessing_page::GuessingPage;
use super::login::Login;
use super::signup::Signup;
use crate::entities::interfaces::User;

#[derive(Clone, PartialEq, Debug)]
pub struct StringWrap {
    pub cat: String,
}

impl Display for StringWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cat.fmt(f)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseStringWrapError;

impl FromStr for StringWrap {
    type Err = ParseStringWrapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringWrap{cat: s.to_string()})
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    LaunchPage,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/dummy")]
    GuessingPageDummy,
    #[at("/guess/:opt_str")]
    GuessingPage {opt_str: StringWrap},
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
                Route::LaunchPage => {
                    html! {
                        <LaunchPage />
                    }
                }
                // Route::GuessingPageDefault => {
                //     let opt_user = state.opt_user.clone();
                //     let opt_cat: Option<String> = None;
                //     html! {
                //         <GuessingPage {opt_user} {opt_cat}/>
                //     }
                // },
                Route::GuessingPage {opt_str} => {

                    let opt_user = state.opt_user.clone();
                    let opt_cat = if opt_str.cat == "Default" {
                        None
                    } else {
                        Some(opt_str.cat.clone())
                    };
                    let dummy = false;
                    html! {
                        <GuessingPage {opt_user} {opt_cat} {dummy}/>
                    }
                },
                Route::GuessingPageDummy => {
                    let opt_user = state.opt_user.clone();
                    let opt_cat: Option<String> = None;
                    let dummy = true;
                    html! {
                        <GuessingPage {opt_user} {opt_cat} {dummy}/>
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
