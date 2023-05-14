use crate::components::app::Route;
use crate::entities::interfaces::{Article, Status};
use crate::service::articles::get_matches;
use crate::service::future::handle_future;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;

struct PreparationPageState {
    input_title_search: String,
    potential_articles: Vec<Article>,
    sel_article: Option<Article>,
    sel_link: Option<String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PreparationProps {
    pub cb_route: Callback<Route>,
    pub route: Route,
}

#[function_component(PreparationPage)]
pub fn preparation_page(props: &PreparationProps) -> Html {
    let state = use_state(|| PreparationPageState {
        input_title_search: "".to_string(),
        potential_articles: Vec::new(),
        sel_article: None,
        sel_link: None,
    });

    if props.route != Route::Preparation {
        props.cb_route.emit(Route::Preparation);
    }

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
            state.set(PreparationPageState {
                input_title_search: value.clone(),
                potential_articles: suggestion_empty,
                sel_article: opt_article.cloned(),
                sel_link,
            });
            log::info!("Sel article: {:?}", state.sel_article);
            let state_bis = state.clone();
            let value_bis = target.value();
            if opt_article.is_none() {
                if value.len() >= 3 {
                    let future = async move { get_matches(&value.clone()).await };
                    handle_future(future, move |data: Result<Vec<Article>, Status>| {
                        match data {
                            Ok(articles) => {
                                state_bis.set(PreparationPageState {
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
            }
        })
    };
    html! {
        <div class="row">
            <div class="wikitty" title="Trouvons une page ensemble !">
                <img class="logo" src="img/wikitty_v1.png"/>
            </div>
            <div class="welcome">
                <h1 class="welcome_title">{"Mode défi"}</h1>
                <p class="welcome_desc">
                    {"Lance un défi à un(e) ami(e) ! Prépare une page wikipédia à trouver, et partage lui le lien."}
                </p>
                <div style="display: flex; flex-direction: column">
                    <input class="input title" type="text" placeholder="Ex: Télévision" oninput={input_search} value={state.input_title_search.clone()} id="input_search" name="input_search" list="article_suggestion" size=10/>
                    <datalist id="article_suggestion">
                    {
                        state.potential_articles.iter().map(|article| {
                            let title = article.title.clone();
                            html! { <option value={title.clone()}> {title.clone()} </option> }
                        }).collect::<Html>()
                    }
                    </datalist>
                    {
                        if let Some(link) = state.sel_link.clone() {
                            html!{<p class="link"> {link} </p>}
                        } else {
                            html!{}
                        }
                    }
                </div>
            </div>
        </div>
    }
}

fn get_link(id: i32) -> String {
    format!("www.wikitrouve.fr/guess/{}", id).to_string()
}
