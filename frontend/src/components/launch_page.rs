use crate::entities::interfaces::Status;
use crate::service::future::handle_future;
use crate::service::games::get_ongoing_game;
use crate::{components::app::Route, entities::interfaces::Game};
use web_sys::{HtmlInputElement, HtmlSelectElement, InputEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use super::app::StringWrap;

struct LaunchPageState {
    cat: String,
    opt_game: Option<Option<Game>>,
    article_id: String,
}

#[function_component(LaunchPage)]
pub fn launch_page() -> Html {
    let state = use_state(|| LaunchPageState {
        cat: "Default".to_string(),
        opt_game: None,
        article_id: "".to_string(),
    });

    let history = use_history().unwrap();
    let onclick_launch_id = {
        let history = history.clone();
        let state = state.clone();
        Callback::from(move |_| {
            if let Ok(_) = state.article_id.parse::<i32>() {
                // TODO: check article exists
                // launch guessing page
                let opt_str = StringWrap {
                    cat_or_id: state.article_id.clone(),
                };
                history.push(Route::GuessingPage { opt_str });
            }
        })
    };
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
            state.set(LaunchPageState {
                cat,
                opt_game: state.opt_game.clone(),
                article_id: state.article_id.clone(),
            });
        })
    };
    let oninput_id = {
        let state = state.clone();
        Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_unchecked_into();
            let value = target.value();
            log::info!("target: {:?}", target);
            log::info!("value: {:?}", value);
            let article_id = value.clone();
            state.set(LaunchPageState {
                cat: state.cat.clone(),
                opt_game: state.opt_game.clone(),
                article_id,
            });
        })
    };
    // check if there is an ongoing game, switch to this if there is
    let future = async move { get_ongoing_game().await };
    handle_future(future, move |data: Result<Option<Game>, Status>| {
        match data {
            Ok(opt_ongoig) => {
                log::info!("Game ongoing: {:?}", opt_ongoig);
                if let Some(_ongoing) = opt_ongoig {
                    history.push(Route::GuessingPage {
                        opt_str: StringWrap {
                            cat_or_id: "None".to_string(),
                        },
                    });
                }
            }
            Err(_) => {
                log::info!("Error loading the data !");
            }
        };
    });
    let string_launch_button = "Get a random page !".to_string();
    // let string_launch_button = "Get a random page !".to_string() + &state.cat.clone();
    html! {
        <div >
            <div style="display: flex">
                <select onchange={onchange_cat}>
                    <option value="Default" selected=true>{ "Pas de filtres" }</option>
                    <option value="Geographie">{ "Géographie" }</option>
                    <option value="Histoire">{ "Histoire" }</option>
                    <option value="Sciences">{ "Science" }</option>
                    <option value="Sport">{ "Sport" }</option>
                    <option value="Culture&Religion">{ "Culture et religion" }</option>
                    <option value="Art&Loisir">{ "Art et loisirs" }</option>
                </select>
                <button class="launch" onclick={onclick_launch_cat}>
                    {
                        string_launch_button
                    }
                </button>
            </div>
            <p> {"       or..       "}</p>
            <div style="display: flex">
                <input type="text" oninput={oninput_id} value={state.article_id.clone()} id="input_id" name="input_id" size=10/>
                {
                    html! { <button class="launch" onclick={onclick_launch_id}> { "Find page with id" } </button> }
                }
            </div>
        </div>
    }
}
