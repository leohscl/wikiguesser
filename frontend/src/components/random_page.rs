use crate::{components::app::Route, entities::interfaces::Game};
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::prelude::*;

use super::app::StringWrap;

struct RandomPageState {
    cat: String,
    opt_game: Option<Option<Game>>,
}

#[function_component(RandomPage)]
pub fn random_page() -> Html {
    let state = use_state(|| RandomPageState {
        cat: "Default".to_string(),
        opt_game: None,
    });

    let history = use_history().unwrap();
    let onclick_launch_cat = {
        let history = history.clone();
        let state = state.clone();
        Callback::from(move |_| {
            let opt_str = StringWrap {
                cat_or_id: state.cat.clone(),
            };
            history.push(Route::GuessingPage { opt_str });
        })
    };
    let onchange_cat = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let target: HtmlSelectElement = e.target_unchecked_into();
            let value = target.value();
            log::info!("target: {:?}", target);
            log::info!("value: {:?}", value);
            let cat = value.clone();
            state.set(RandomPageState {
                cat,
                opt_game: state.opt_game.clone(),
            });
        })
    };
    let string_launch_button = "Nouvelle page aléatoire".to_string();
    html! {
        <div class="row">
            <div class="wikitty" title="Trouvons une page ensemble !">
                <img class="logo" src="https://upload.wikimedia.org/wikipedia/commons/6/61/Wikipedia-logo-transparent.png"/>
            </div>
            <div class="welcome">
                <h1 class="welcome_title">{"Obtenir une page aléatoire"}</h1>
                <p class="welcome_desc">
                    {"Tire une page au hasard et essaie de la deviner ! Tu peux choisir une catégorie de page avant de te lancer, ou alors avoir une page complètement aléatoire."}
                </p>
                <div class="dropdown dropdown-lang">
                </div>
                <div style="display: flex">
                    <div class="select">
                        <select onchange={onchange_cat}>
                            <option value="Default" selected=true>{ "Pas de filtres" }</option>
                            <option value="Geographie">{ "Géographie" }</option>
                            <option value="Histoire">{ "Histoire" }</option>
                            <option value="Sciences">{ "Science" }</option>
                            <option value="Sport">{ "Sport" }</option>
                            <option value="Culture&Religion">{ "Culture et religion" }</option>
                            <option value="Art&Loisir">{ "Art et loisirs" }</option>
                        </select>
                    </div>
                    <button class="launch padding" onclick={onclick_launch_cat}>
                        {
                            string_launch_button
                        }
                    </button>
                </div>
            </div>
        </div>
    }
}
