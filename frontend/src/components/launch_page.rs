use crate::entities::interfaces::{Article, Status};
use crate::service::future::handle_future;
use crate::service::games::get_ongoing_game;
use crate::{components::app::Route, entities::interfaces::Game};
use web_sys::{HtmlInputElement, HtmlSelectElement, InputEvent};
use yew::prelude::*;
use yew_hooks::use_clipboard;
use yew_hooks::UseClipboardHandle;
use yew_router::prelude::*;

use super::app::StringWrap;

struct LaunchPageState {
    cat: String,
    opt_game: Option<Option<Game>>,
    article_id: String,
    input_title_search: String,
    potential_articles: Vec<Article>,
    sel_index: Option<usize>,
}

#[function_component(LaunchPage)]
pub fn launch_page() -> Html {
    let state = use_state(|| LaunchPageState {
        cat: "Default".to_string(),
        opt_game: None,
        article_id: "".to_string(),
        input_title_search: "".to_string(),
        potential_articles: Vec::new(),
        sel_index: None,
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
    let clipboard: UseClipboardHandle = use_clipboard();
    let onclick_get_link = {
        let clipboard = clipboard;
        Callback::from(move |_| {
            clipboard.write_text("hello world!".to_owned());
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
                input_title_search: value,
                potential_articles: Vec::new(),
                sel_index: None,
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
                input_title_search: state.input_title_search.clone(),
                potential_articles: Vec::new(),
                sel_index: None,
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

    let input_search = {
        let state = state.clone();
        Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_unchecked_into();
            let value = target.value();
            let mut suggestion_test = Vec::new();
            suggestion_test.push(Article::dummy());
            state.set(LaunchPageState {
                cat: state.cat.clone(),
                opt_game: state.opt_game.clone(),
                article_id: state.article_id.clone(),
                input_title_search: value,
                potential_articles: suggestion_test,
                sel_index: None,
            });
        })
    };
    html! {
        <div >
            <div style="display: flex">
                <select class="cat_sel" onchange={onchange_cat}>
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
                <button class="launch" onclick={onclick_launch_id}> { "Find page with id" } </button>
            </div>
            <p> {"    Get page id !    "}</p>
            <div style="display: flex">
                <input type="text" oninput={input_search} value={state.input_title_search.clone()} id="input_search" name="input_search" list="article_suggestion" size=10/>
                <datalist id="article_suggestion">
                {
                    state.potential_articles.iter().map(|article| {
                        let title = article.title.clone();
                        html! { <option value={title.clone()}> {title.clone()} </option> }
                    }).collect::<Html>()
                }
                </datalist>
                <button class="launch" onclick={onclick_get_link}> { "Get link !" } </button>
            </div>
        </div>
    }
}
