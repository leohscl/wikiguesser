use crate::entities::interfaces::{Article, Status};
use crate::service::articles::get_matches;
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
    input_title_search: String,
    potential_articles: Vec<Article>,
    sel_article: Option<Article>,
    sel_link: Option<String>,
}

#[function_component(LaunchPage)]
pub fn launch_page() -> Html {
    let state = use_state(|| LaunchPageState {
        cat: "Default".to_string(),
        opt_game: None,
        article_id: "".to_string(),
        input_title_search: "".to_string(),
        potential_articles: Vec::new(),
        sel_article: None,
        sel_link: None,
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
    // let clipboard: UseClipboardHandle = use_clipboard();
    // let onclick_get_link = {
    //     let clipboard = clipboard;
    //     let state = state.clone();
    //     Callback::from(move |_| {
    //         let article = state
    //             .sel_article
    //             .clone()
    //             .expect("There should be an article now");
    //         let id = article.id;
    //         let link = get_link(id);
    //         clipboard.write_text(link);
    //         // make toast to say text copied ?
    //     })
    // };
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
                sel_article: state.sel_article.clone(),
                sel_link: state.sel_link.clone(),
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
                sel_article: state.sel_article.clone(),
                sel_link: state.sel_link.clone(),
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
            let suggestion_empty = Vec::new();
            // suggestion_test.push(Article::dummy());
            let opt_article = state
                .potential_articles
                .iter()
                .find(|article| article.title == value);
            log::info!("opt article: {:?}", opt_article);
            let sel_link = if let Some(article) = opt_article.clone() {
                Some(get_link(article.id))
            } else {
                None
            };
            let sel_link_bis = sel_link.clone();
            state.set(LaunchPageState {
                cat: state.cat.clone(),
                opt_game: state.opt_game.clone(),
                article_id: state.article_id.clone(),
                input_title_search: value.clone(),
                potential_articles: suggestion_empty,
                sel_article: opt_article.cloned(),
                sel_link,
            });
            let state_bis = state.clone();
            let value_bis = target.value();
            if value.len() >= 3 {
                let future = async move { get_matches(&value.clone()).await };
                handle_future(future, move |data: Result<Vec<Article>, Status>| {
                    match data {
                        Ok(articles) => {
                            state_bis.set(LaunchPageState {
                                cat: state_bis.cat.clone(),
                                opt_game: state_bis.opt_game.clone(),
                                article_id: state_bis.article_id.clone(),
                                input_title_search: value_bis.clone(),
                                potential_articles: articles,
                                sel_article: None,
                                sel_link: sel_link_bis.clone(),
                            });
                        }
                        Err(_) => {
                            log::info!("Error loading the data !");
                        }
                    };
                });
            }
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
            <p> {"    Share wikipedia page with your friend !    "}</p>
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
            </div>
            {
                if let Some(link) = state.sel_link.clone() {
                    html!{<p> {link} </p>}
                } else {
                    html!{}
                }
            }
        </div>
    }
}

fn get_link(id: i32) -> String {
    format!("www.wikitrouve.fr/guess/{}", id).to_string()
}
