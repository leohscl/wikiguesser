use yew::prelude::*;
use yew_router::prelude::*;
use crate::service::games::get_ongoing_game;
use crate::{components::app::Route, entities::interfaces::Game};
use crate::entities::interfaces::Status;
use web_sys::HtmlSelectElement;
use crate::service::future::handle_future;

use super::app::StringWrap;

struct LaunchPageState {
    cat: String,
    opt_game: Option<Option<Game>>,
}

#[function_component(LaunchPage)]
pub fn launch_page() -> Html {
    let state = use_state(|| LaunchPageState{cat: "Default".to_string(), opt_game: None});
    let history = use_history().unwrap();
    let onclick_launch = {
        let history = history.clone();
        let state = state.clone();
        Callback::from( move |_| {
            let opt_str = StringWrap{cat: state.cat.clone()};
            history.push(Route::GuessingPage { opt_str });
        })
    };
    let onchange = {
        let state = state.clone();
        Callback::from( move |e: Event | {
            let target: HtmlSelectElement = e.target_unchecked_into();
            let value = target.value();
            log::info!("target: {:?}", target);
            log::info!("value: {:?}", value);
            let cat = value.clone();
            state.set(LaunchPageState{cat, opt_game: state.opt_game.clone()});
        })
    };
    let future = async move { get_ongoing_game().await };
    handle_future(future, move |data: Result<Option<Game>, Status>| {
        match data {
            Ok(opt_ongoig) => {
                log::info!("Game ongoing: {:?}", opt_ongoig);
                if let Some(_ongoing) = opt_ongoig {
                    history.push(Route::GuessingPage { opt_str: StringWrap { cat: "None".to_string() }});
                }
            }
            Err(_) => {
                log::info!("Error loading the data !");
            },
        };
    });
    let string_launch_button = "Get a random page !".to_string();
    // let string_launch_button = "Get a random page !".to_string() + &state.cat.clone();
    html! {
        <div>
            <select onchange={onchange}>
                <option value="Default" selected=true>{ "Pas de filtres" }</option>
                <option value="Geographie">{ "Géographie" }</option>
                <option value="Histoire">{ "Histoire" }</option>
                <option value="Sciences">{ "Science" }</option>
                <option value="Sport">{ "Sport" }</option>
                <option value="Culture&Religion">{ "Culture et religion" }</option>
                <option value="Art&Loisir">{ "Art et loisirs" }</option>
            </select>
            <button onclick={onclick_launch}>
                { 
                    string_launch_button
                }
            </button>
        </div>
    }
}
