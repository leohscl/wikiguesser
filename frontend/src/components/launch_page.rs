use crate::components::app::Route;
use crate::components::app::StringWrap;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(LaunchPage)]
pub fn launch_page() -> Html {
    let history = use_history().unwrap();
    let onclick_launch_daily = {
        let history = history.clone();
        // let state_copy = state.clone();
        let wrap_daily = StringWrap {
            cat_or_id: "Daily".to_string(),
        };
        Callback::from(move |_| {
            // if let Some(opt_game) = state_copy.opt_game.clone() {
            // log::info!("Ongoing game: {:?}", opt_game);
            // }
            history.push(Route::Guessing {
                opt_str: wrap_daily.clone(),
            });
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
