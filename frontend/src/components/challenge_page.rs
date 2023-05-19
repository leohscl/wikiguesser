use crate::components::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeProps {
    pub cb_route: Callback<Route>,
    pub route: Route,
}

#[function_component(ChallengePage)]
pub fn challenge_page(props: &ChallengeProps) -> Html {
    if props.route != Route::ChallengePage {
        props.cb_route.emit(Route::ChallengePage);
    }

    let history = use_history().unwrap();
    let onclick_launch_challenge = {
        let history = history.clone();
        Callback::from(move |_| {
            history.push(Route::Challenge {
                opt_str: super::app::StringWrap {
                    cat_or_id: "challenge".to_string(),
                },
            });
        })
    };
    let string_launch_button = "C'est parti !".to_string();
    html! {
        <div class="row">
            <div class="wikitty" title="Page aléatoire">
                <img class="logo" src="https://upload.wikimedia.org/wikipedia/commons/6/61/Wikipedia-logo-transparent.png"/>
            </div>
            <div class="welcome">
                <h1 class="welcome_title">{"Défi"}</h1>
                <p class="welcome_desc">
                    {"Tire une page au hasard et essaie de la deviner ! Ton temps est limité, mais tu as des indices pour trouver la page plus facilement."}
                </p>
                <div class="dropdown dropdown-lang">
                </div>
                <div style="display: flex">
                    <button class="launch" onclick={onclick_launch_challenge}>
                        {
                            string_launch_button
                        }
                    </button>
                </div>
            </div>
        </div>
    }
}
