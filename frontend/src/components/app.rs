use std::fmt::Display;
use std::str::FromStr;

use yew::prelude::*;
use yew_router::prelude::*;

use super::challenge_page::ChallengePage;
use super::guessing_page::GuessingPage;
use super::guessing_page::Mode;
use super::guessing_page::Prereveal;
use super::guessing_page::TimeConstraint;
use super::information_page::InformationPage;
use super::launch_page::LaunchPage;
use super::login::Login;
use super::preparation_page::PreparationPage;
use super::random_page::RandomPage;
use super::report_page::ReportPage;
use super::roadmap_page::RoadMap;
use super::signup::Signup;
use crate::entities::interfaces::User;

#[derive(Clone, PartialEq, Debug)]
pub struct StringWrap {
    pub cat_or_id: String,
}

impl Display for StringWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cat_or_id.fmt(f)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseStringWrapError;

impl FromStr for StringWrap {
    type Err = ParseStringWrapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringWrap {
            cat_or_id: s.to_string(),
        })
    }
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    LaunchPage,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/report/:article_id")]
    ReportForm { article_id: i32 },
    #[at("/dummy")]
    GuessingPageDummy,
    #[at("/preparation")]
    Preparation,
    #[at("/information")]
    Information,
    #[at("/guess")]
    RandomPage,
    #[at("/guess/:opt_str")]
    Guessing { opt_str: StringWrap },
    #[at("/challenge/:opt_str")]
    Challenge { opt_str: StringWrap },
    #[at("/challenge")]
    ChallengePage,
    #[at("/challenge/roadmap")]
    RoadMap,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn get_selection(&self) -> (String, String, String, String, String) {
        let e0 = "".to_string();
        let e1 = "".to_string();
        let e2 = "".to_string();
        let e3 = "".to_string();
        let e4 = "".to_string();
        let sel = "selected_nav".to_string();
        match self {
            Self::ChallengePage => (sel, e1, e2, e3, e4),
            Self::Preparation => (e0, e1, e2, sel, e4),
            Self::Information => (e0, e1, e2, e3, sel),
            Self::RandomPage => (e0, e1, sel, e3, e4),
            Self::Guessing { opt_str } => {
                if opt_str.cat_or_id == "daily" {
                    (e0, sel, e2, e3, e4)
                } else if opt_str.cat_or_id == "challenge" {
                    (sel, e1, e2, e3, e4)
                } else {
                    (e0, e1, sel, e3, e4)
                }
            }
            _ => (e0, e1, e2, e3, e4),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct AppState {
    opt_user: Option<User>,
    current_route: Route,
}

fn user_logged_in(appstate: &AppState) -> bool {
    match appstate.opt_user {
        Some(_) => true,
        None => false,
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| AppState {
        opt_user: None,
        current_route: Route::LaunchPage,
    });
    let cb_user_login: Callback<User> = {
        let state = state.clone();
        Callback::from(move |user: User| {
            let greeting = format!("Hey, {} !", user.t_email);
            log::info!("{}", greeting);
            state.set(AppState {
                opt_user: Some(user),
                current_route: state.current_route.clone(),
            });
        })
    };

    let state_clone = state.clone();
    let cb_route = Callback::from(move |route: Route| {
        state_clone.set(AppState {
            opt_user: state_clone.opt_user.clone(),
            current_route: route.clone(),
        });
    });

    let switch = {
        let state = state.clone();
        Switch::render(move |routes: &Route| {
            let cb_route = cb_route.clone();
            let route = state.current_route.clone();
            // let route = routes.clone();
            log::info!("Route: {:?}", routes);
            match routes {
                Route::Information => {
                    html! {
                    <InformationPage {cb_route} {route} />
                    }
                }
                Route::Signup => html! {
                    <Signup />
                },
                Route::Login => {
                    let cb_user_login = cb_user_login.clone();
                    html! {
                        <Login {cb_user_login} />
                    }
                }
                Route::RandomPage => {
                    html! {
                        <RandomPage {cb_route} {route} />
                    }
                }
                Route::Preparation => {
                    html! {
                        <PreparationPage {cb_route} {route} />
                    }
                }
                Route::LaunchPage => {
                    html! {
                        <LaunchPage />
                    }
                }
                Route::ReportForm { article_id } => {
                    let article_id = article_id.clone();
                    html! {
                        <ReportPage {article_id}/>
                    }
                }
                Route::Guessing { opt_str } => {
                    let opt_user = None;
                    let mode = match opt_str.cat_or_id.as_str() {
                        "Daily" => Mode::Daily,
                        _ => Mode::Random,
                    };

                    let (opt_cat, opt_id) = if let Ok(id) = opt_str.cat_or_id.parse::<i32>() {
                        (None, Some(id))
                    } else {
                        let opt_cat = if opt_str.cat_or_id == "Default" {
                            None
                        } else {
                            Some(opt_str.cat_or_id.clone())
                        };
                        (opt_cat, None)
                    };
                    let dummy = false;
                    let constraint = TimeConstraint::Unconstrained;
                    let prereveal = Prereveal::Base;
                    html! {
                        <GuessingPage {opt_user} {opt_cat} {opt_id} {dummy} {mode} {cb_route} {route} {constraint} {prereveal} />
                    }
                }
                Route::ChallengePage => {
                    html! {
                        <ChallengePage {cb_route} {route}/>
                    }
                }
                Route::RoadMap => {
                    html! {
                        <RoadMap />
                    }
                }
                Route::Challenge { opt_str } => {
                    let opt_user = None;
                    let mode = Mode::Challenge;

                    let opt_id = if let Ok(id) = opt_str.cat_or_id.parse::<i32>() {
                        Some(id)
                    } else {
                        None
                    };
                    let dummy = false;
                    let constraint = TimeConstraint::Constraint(300);
                    let prereveal = Prereveal::OverAndHintUnder(100.0, 10.0);
                    let opt_cat: Option<String> = None;
                    html! {
                        <GuessingPage {opt_user} {opt_cat} {opt_id} {dummy} {mode} {cb_route} {route} {constraint} {prereveal} />
                    }
                }
                Route::GuessingPageDummy => {
                    // let opt_user = state.opt_user.clone();
                    let opt_user = None;
                    let opt_cat: Option<String> = None;
                    let opt_id: Option<i32> = None;
                    let mode = Mode::Challenge;
                    let dummy = true;
                    let constraint = TimeConstraint::Unconstrained;
                    let prereveal = Prereveal::Base;
                    html! {
                        <GuessingPage {opt_user} {opt_cat} {opt_id} {dummy} {mode} {cb_route} {route} {constraint} {prereveal} />
                    }
                }
                Route::NotFound => html! { <h1>{ "404" }</h1> },
            }
        })
    };
    let wrap_daily = StringWrap {
        cat_or_id: "Daily".to_string(),
    };
    let tuple_selected = state.current_route.get_selection();
    html! {
        <>
            <BrowserRouter>
                <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">
                        <Link<Route> classes={classes!("wikitrouve")} to={Route::LaunchPage}>
                            { "WIKITROUVE" }
                        </Link<Route>>
                        <div class="navbar-start">
                        {
                            if user_logged_in(&state) {
                                html!{
                                }
                            } else {
                                html!{
                                    <ul class="nav navbar-nav">
                                    <li class="nav-item" id="tutorMenuItem">
                                        <Link<Route> classes={classes!("navbar-item", tuple_selected.0)} to={Route::ChallengePage}>
                                            { "Défi" }
                                        </Link<Route>>
                                    </li>
                                    <li class="nav-item" id="tutorMenuItem">
                                        <Link<Route> classes={classes!("navbar-item", tuple_selected.1)} to={Route::Guessing{ opt_str: wrap_daily }}>
                                            { "Page du jour" }
                                        </Link<Route>>
                                    </li>
                                    <li class="nav-item ">
                                        <Link<Route> classes={classes!("navbar-item", tuple_selected.2)} to={Route::RandomPage}>
                                            { "Page aléatoire" }
                                        </Link<Route>>
                                    </li>
                                    <li class="nav-item active">
                                        <Link<Route> classes={classes!("navbar-item", tuple_selected.3)} to={Route::Preparation}>
                                            { "Préparation de page" }
                                        </Link<Route>>
                                    </li>
                                    <li class="nav-item active">
                                        <Link<Route> classes={classes!("navbar-item", tuple_selected.4)} to={Route::Information}>
                                            { "Informations" }
                                        </Link<Route>>
                                    </li>
                                    </ul>
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
