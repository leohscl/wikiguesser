use crate::{components::app::Route, entities::interfaces::Game};
use yew::prelude::*;
use yew_router::prelude::*;

struct LaunchPageState {
    opt_game: Option<Option<Game>>,
}

#[function_component(LaunchPage)]
pub fn launch_page() -> Html {
    let state = use_state(|| LaunchPageState { opt_game: None });
    let opt_location = use_location();
    if let Some(any_loc) = opt_location {
        let route = any_loc.route::<Route>();
        log::info!("Route: {:?}", route);
    }

    let history = use_history().unwrap();
    let onclick_launch_daily = {
        let history = history.clone();
        let state_copy = state.clone();
        Callback::from(move |_| {
            if let Some(opt_game) = state_copy.opt_game.clone() {
                log::info!("Ongoing game: {:?}", opt_game);
            }
            history.push(Route::RandomPage);
        })
    };
    let string_launch_button = "Page du jour".to_string();
    // <img class="logo" src="https://upload.wikimedia.org/wikipedia/commons/6/61/Wikipedia-logo-transparent.png"/>
    html! {
        <div class="row">
            <div class="wikitty" title="Trouvons une page ensemble !">
                <img class="logo" src="img/wikitty_v1.png"/>
            </div>
            <div class="welcome">
                <h1 class="welcome_title">{"Trouve la page wikipédia cachée"}</h1>
                <p class="welcome_desc">
                    {"Devine le titre de la page, un mot à la fois ! Le jeu t'aideras, si ton mot est proche de l'un des mots cachés, il s'affichera."}
                </p>
                <div style="display: flex">
                    <button class="launch daily" onclick={onclick_launch_daily}>
                        {
                            string_launch_button
                        }
                    </button>
                </div>
            </div>
        </div>
    }
}
