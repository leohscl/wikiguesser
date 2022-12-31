use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::app::Route;
use web_sys::HtmlSelectElement;

use super::app::StringWrap;

struct LaunchPageState {
    cat: String,
}

#[function_component(LaunchPage)]
pub fn launch_page() -> Html {
    let state = use_state(|| LaunchPageState{cat: "Default".to_string()});
    let history = use_history().unwrap();
    let onclick_launch = {
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
            state.set(LaunchPageState{cat});
        })
    };
    let string_launch_button = "Get a random page !".to_string();
    // let string_launch_button = "Get a random page !".to_string() + &state.cat.clone();
    html! {
        <div>
            <select onchange={onchange}>
                <option value="Default" selected=true>{ "Pas de filtres" }</option>
                <option value="Geographie">{ "Géographie" }</option>
                <option value="Histoire">{ "Histoire" }</option>
                <option value="Science">{ "Science" }</option>
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
